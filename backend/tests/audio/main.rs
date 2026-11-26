use std::process::Command;
use futures_util::SinkExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use parakeet_rs::ParakeetTDT;
use std::error::Error;
use std::io::Result;

#[tokio::test]
async fn mic_test() {
    let output = Command::new("arecord")
        .args([
            "-q",
            "-d", "2",
            "-t", "raw",
            "-f", "S16_LE",
            "-r", "16000",
            "-c", "1"
        ])
        .output()
        .expect("Could not start arecord.");
    
    assert!(output.status.success(), "arecord failed: {}", String::from_utf8_lossy(&output.stderr));

    let audio = output.stdout;
    assert_eq!(audio.len(), 64_000);

    let (mut socket, _) = connect_async("ws://localhost:3000/audio")
        .await
        .expect("Could not connect to audio websocket");

    socket.send(Message::Binary(audio.into()))
        .await.expect("Could not send audio");

    socket.close(None).await.expect("Couldn't close socket");
}
