#[derive(serde::Deserialize)]
struct LogResponse {
    id: String,
}

#[test]
fn get_log() {
    let id = "11111111-1111-4111-8111-111111111111";
    let url = format!("http://localhost:3000/logs/{}", id);
    let response = std::process::Command::new("curl")
        .args(["-fsS", url.as_str()])
        .output()
        .expect("could not start curl");

    assert!(response.status.success(), "curl failed");
    let parsed: LogResponse = serde_json::from_slice(&response.stdout)
        .expect("expected a JSON object with a string id");
    assert_eq!(parsed.id, id);
}
