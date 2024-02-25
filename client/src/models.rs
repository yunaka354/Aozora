use serde::{Deserialize, Serialize};

// Response from app.bsky.feed.getTimeline
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timeline {
    pub cursor: String,
    pub feed: Vec<Feed>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feed {
    pub post: Post,
    pub reason: Option<Reason>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub author: Author,
    pub cid: String,
    pub indexed_at: String,
    pub like_count: u32,
    pub record: Record,
    pub reply_count: u32,
    pub repost_count: u32,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub avatar: String,
    pub did: String,
    pub display_name: String,
    pub handle: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub record_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Reason {
    pub record_type: String,
    pub by: By,
    pub indexed_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct By {
    pub avatar: String,
    pub did: String,
    pub display_name: String,
    pub handle: String,
    pub labels: Vec<String>,
    pub viewer: Viewer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Viewer {
    pub blocked_by: bool,
    pub followed_by: String,
    pub following: String,
    pub muted: bool,
}
