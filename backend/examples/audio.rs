use axum::http::request;
use backend::utils::{transcribe, build_model};
use std::process::Command;
use parakeet_rs::ParakeetTDT;
use reqwest::Client;
use serde_json::{Value, json};
use std::{collections::BTreeMap, error::Error};

fn record_mic(seconds: &str) -> Vec<u8> {
    Command::new("arecord")
        .args([
            "-q",
            "-d", seconds,
            "-t", "raw",
            "-f", "S16_LE",
            "-r", "16000",
            "-c", "1"
        ])
        .output()
        .expect("Could not start arecord.")
        .stdout
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
               "boundary": {
                   "type": "choice",
                   "instructions": instructions,
                   "criteria": choices
               }
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

    for n in 0..=buffer.len() {
        choices.insert(n.to_string(), format!("The first entry ends at index: {n}"));
    }

    let state = json!({"current buffer state": buffer});
    let instructions = "Here is a buffer of words that is
        building up due to a person talking into a microphone.
        Choose the index that corresponds to the final word in
        a complete food entry that could be searched to figure
        out its calories and protein.";
    let result = jev_choice(&choices, state, instructions).await?;
    Ok(Some(result))
}

async fn mic_test(model: &mut ParakeetTDT, seconds: &str) -> Result<(), AppError> {
    let seconds_i32: i32 = seconds.parse::<i32>().unwrap();
    println!("[AUDIO] speak now for {} seconds", seconds);
    
    let audio = record_mic(seconds);
    assert_eq!(audio.len(), (32_000_i32 * seconds_i32) as usize);

    let text = transcribe(model, &audio).expect("transcription failed");
    println!("[AUDIO] {}", text);

    let mut buffer: Vec<String> = text.split_whitespace().map(String::from).collect();
    let Some(cutoff) = entry_index_or_wait(&buffer).await? else {  
        return Ok(())
    };

    if cutoff == "wait" {
        println!("Returned wait");
        return Ok(());
    }

    let index = cutoff.parse()?;
    let entry = buffer.get(..=index).ok_or("Jev returned an invalid index")?.join(" ");
    buffer.drain(..=index);
    println!("Entry: {}", entry);
    println!("New buffer: {:?}", buffer);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let seconds = "10";
    let mut model = build_model();
    mic_test(&mut model, seconds).await
}
