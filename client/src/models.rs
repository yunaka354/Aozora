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
    pub created_at: String,
    pub text: String,
}
