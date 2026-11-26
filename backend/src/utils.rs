use parakeet_rs::{ParakeetTDT, Transcriber};
use std::error::Error;

pub fn transcribe(
    model: &mut ParakeetTDT,
    audio: &[u8]) -> Result<String, Box<dyn Error>> {
    if !audio.len().is_multiple_of(2) {
        return Err("incomplete 16-bit audio sample".into());
    }

    let samples: Vec<f32> = audio
        .as_chunks::<2>().0.iter()
        .map(|pair| i16::from_le_bytes([pair[0], pair[1]]) as f32 / 32768.0)
        .collect();

    let result = model.transcribe_samples(samples, 16_000, 1, None)?;
    Ok(result.text)
}

pub fn build_model() -> ParakeetTDT {
    let env = parakeet_rs::ort::init()
        .build()
        .expect("Could not initialize ONNX runtime");

    let config = parakeet_rs::ExecutionConfig::new().with_environment(env);

    ParakeetTDT::from_pretrained(
        "models/parakeet-tdt-0.6b-v3", Some(config))
        .expect("could not load Parakeet")
}
