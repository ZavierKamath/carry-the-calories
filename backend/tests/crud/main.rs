use chrono::Utc;
use backend::models::Log;

#[test]
fn post_get_delete() {
    fn post_log() -> uuid::Uuid {
        let url = "http://localhost:3000/logs";
        let body = serde_json::json!({
            "datetime": Utc::now(),
            "name": "chicken breast",
            "calories": 123.4,
            "protein": 18.3,
        }).to_string();

        let response = std::process::Command::new("curl")
            .args(["-fsS", "--json", body.as_str(), url])
            .output()
            .expect("could not start curl");

        assert!(response.status.success(), "curl failed {}", String::from_utf8_lossy(&response.stderr));
        let parsed: Log = serde_json::from_slice(&response.stdout)
            .expect("expected a JSON object with a string id");

        parsed.id
    }

    fn get_log(id: &str) {
        let url = format!("http://localhost:3000/logs/{}", id);
        let response = std::process::Command::new("curl")
            .args(["-fsS", url.as_str()])
            .output()
            .expect("could not start curl");

        assert!(response.status.success(), "curl failed {}", String::from_utf8_lossy(&response.stderr));
        let parsed: Log = serde_json::from_slice(&response.stdout)
            .expect("expected a JSON object with a string id");
        assert_eq!(parsed.id.to_string(), id);
        assert!(parsed.grams.is_none(), "Expected grams to be None");
    }

    fn del_log(id: &str) {
        let url = format!("http://localhost:3000/logs/{}", id);
        let response = std::process::Command::new("curl")
            .args(["-fsS", "-X", "DELETE", url.as_str()])
            .output()
            .expect("could not start curl");

        assert!(response.status.success(), "curl failed {}", String::from_utf8_lossy(&response.stderr));
    }

    let id = post_log().to_string();
    println!("post id: {}", id);
    get_log(&id);
    del_log(&id);
}
