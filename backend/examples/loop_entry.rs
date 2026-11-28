use backend::utils::{transcribe, build_model};
use std::process::{Child, Command, Stdio};
use reqwest::Client;
use serde_json::{Value, json};
use std::{collections::BTreeMap, error::Error};
use std::io::{self, Read};
use tokio::sync::Mutex;
use std::{sync::Arc, thread};
use tokio::time::{sleep, Duration};

fn get_recorder() -> io::Result<Child> {
    Command::new("arecord")
        .args([
            "-q",
            "-t", "raw",
            "-f", "S16_LE",
            "-r", "16000",
            "-c", "1"
        ])
        .stdout(Stdio::piped())
        .spawn()
}

type AppError = Box<dyn Error + Sync + Send>;

async fn jev_choice(
    choices: &BTreeMap<String, String>,
    state: Value,
    instructions: &str
) -> Result<String, AppError> {
    let client = Client::new();
    let key = dotenvy::var("JEV_API_KEY")?;
    let reply: Value = client.post("https://api.typesafe.ai/v1/systemone")
       .bearer_auth(key)
       .json(&json!({
           "model": "jev-latest",
           "state": state,
           "questions": {
               "boundary": {"type": "choice", "instructions": instructions, "criteria": choices}
           }
       }))
       .send().await?
       .error_for_status()?
       .json().await?;

    let choice = reply["answers"]["boundary"]["choice"]
       .as_str()
       .ok_or("Jev response is missing the boundary choice")?;

    Ok(choice.to_owned())
}

async fn entry_index_or_wait(
   buffer: &[String],
) -> Result<Option<String>, AppError> {
    if buffer.is_empty() {
       return Ok(None);
    }

    let mut choices = BTreeMap::new();
    choices.insert("wait".to_owned(), "The first entry needs more text.".to_owned());

    for n in 0..buffer.len() {
        choices.insert(n.to_string(), format!("The first entry ends at index: {n}"));
    }

    let state = json!({"buffer": buffer});
    let instructions = "Here is a buffer of words that is
        building up due to a person talking into a microphone with
        the intent of logging their calories and protein for the day.
        Choose the index that corresponds to the final word in
        a complete food entry that could be researched to figure
        out its calories and protein. Choose wait until you hear
        the start of the next food entry. Once the next entry
        starts, return the index of the previous entry's last word.";

    let result = jev_choice(&choices, state, instructions).await?;
    Ok(Some(result))
}

async fn mic_loop() -> Result<(), AppError> {
    let buffer = Arc::new(Mutex::new(Vec::<String>::new()));
    let mic_buffer = Arc::clone(&buffer);
    let mut entries: Vec<String> = Vec::new();

    thread::spawn(move || {
        let mut model = build_model();
    
        let mut recorder = get_recorder().expect("could not start arecord");
        let mut mic = recorder.stdout.take().expect("stdout was piped");

        let chunk_seconds = 8;
        let mut chunk = vec![0_u8; 32_000 * chunk_seconds];

        println!("[INFO] Recording Started. Ctrl + C to Stop.");

        loop {
            mic.read_exact(&mut chunk).expect("failed to read microphone audio");
            let text = transcribe(&mut model, &chunk).expect("transcription failed");
            println!("[STT] {text:?}");
            mic_buffer.blocking_lock().extend(text.split_whitespace().map(String::from));
        }
    });

    loop {
        sleep(Duration::from_secs(1)).await;
        let snapshot = buffer.lock().await.clone();

        if snapshot.len() < 3 {
            continue
        }

        let Some(cutoff) = entry_index_or_wait(&snapshot).await? else {
            return Ok::<(), AppError>(());
        };

        println!("[JEV] saw {snapshot:?}, chose {cutoff:?}");
        if cutoff == "wait" {
            println!("[INFO] Jev returned wait.");
            continue
        }

        let index = cutoff.parse()?;
        let entry = snapshot.get(..=index).ok_or("Jev returned an invalid index")?.join(" ");
        buffer.lock().await.drain(..=index);

        println!("[INFO] Found Entry: {}.", entry);
        entries.push(entry);
        println!("[INFO] Entries: {:?}.", entries);
    };
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    mic_loop().await
}
