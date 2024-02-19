use crate::model::{CreatePost, Record, Session, Timeline};
use dotenv::dotenv;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;

// struct to centralze all the api calls
#[derive(Clone)]
pub struct API {
    session: Session,
}

impl API {
    pub async fn new() -> API {
        let session = API::get_session().await;
        Self { session }
    }
    // function to save the response to a file
    pub fn save_response(&self, text: String) {
        let json_data: Value = serde_json::from_str(&text).expect("Failed to parse JSON");
        let pretty = serde_json::to_string_pretty(&json_data).expect("Failed to serialize JSON");
        let mut file = File::create("response.json").expect("Failed to create file");
        file.write_all(pretty.as_bytes())
            .expect("Failed to write to file");
    }

    // function to get a session
    pub async fn get_session() -> Session {
        dotenv().ok();
        let bluesky_handle = env::var("BLUESKY_HANDLE").expect("BLUESKY_HANDLE must be set");
        let bluesky_password = env::var("BLUESKY_PASSWORD").expect("BLUESKY_PASSWORD must be set");
        let bsky_url = "https://bsky.social/xrpc/com.atproto.server.createSession";
        let mut map = HashMap::new();
        map.insert("identifier", bluesky_handle);
        map.insert("password", bluesky_password);

        let client = reqwest::Client::new();
        let res = client
            .post(bsky_url)
            .json(&map)
            .send()
            .await
            .expect("Failed to send request");
        let text = res.text().await.expect("Failed to get response text");
        let json_data: Value = serde_json::from_str(&text).expect("Failed to parse JSON");
        serde_json::from_value(json_data).expect("Failed to parse JSON into Session")
    }

    // function to get the timeline
    pub async fn get_timeline(&self) -> Timeline {
        let bsky_url = "https://bsky.social/xrpc/app.bsky.feed.getTimeline";
        let client = reqwest::Client::new();
        let res = client
            .get(bsky_url)
            .header(
                "Authorization",
                format!("Bearer {}", self.session.access_jwt),
            )
            .send()
            .await
            .expect("Failed to send request");
        let text = res.text().await.expect("Failed to get response text");
        self.save_response(text.clone());
        let json_data: Value = serde_json::from_str(&text).expect("Failed to parse JSON");
        serde_json::from_value(json_data).expect("Failed to parse JSON")
    }

    // function to post a tweet
    pub async fn post_tweet(&self, record: Record) -> Value {
        let bsky_url = "https://bsky.social/xrpc/com.atproto.repo.createRecord";
        let client = reqwest::Client::new();

        let create_post = CreatePost {
            repo: self.session.did.clone(),
            collection: "app.bsky.feed.post".to_string(),
            record,
        };
        let res = client
            .post(bsky_url)
            .header(
                "Authorization",
                format!("Bearer {}", self.session.access_jwt),
            )
            .json(&create_post)
            .send()
            .await
            .expect("Failed to send request");
        println!("{}", res.status());
        let text = res.text().await.expect("Failed to get response text");
        println!("{}", text);
        let json_data: Value = serde_json::from_str(&text).expect("Failed to parse JSON");
        serde_json::from_value(json_data).expect("Failed to parse JSON")
    }
}
