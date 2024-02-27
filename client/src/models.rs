use serde::{Deserialize, Serialize};

// Response from app.bsky.feed.getTimeline
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Timeline {
    pub cursor: String,
    pub feed: Vec<Feed>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Feed {
    pub post: Post,
    pub reason: Option<Reason>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Post {
    pub author: Author,
    pub cid: String,
    pub embed: Option<Embed>,
    pub indexed_at: String,
    pub like_count: u32,
    pub record: Record,
    pub reply_count: u32,
    pub repost_count: u32,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Author {
    pub avatar: String,
    pub did: String,
    pub display_name: String,
    pub handle: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    pub followed_by: Option<String>,
    pub following: String,
    pub muted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Embed {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub type_: String,
    pub images: Option<Vec<Image>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Image {
    pub alt: String,
    pub aspect_ratio: AspectRatio,
    pub fullsize: String,
    pub thumb: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AspectRatio {
    pub height: u32,
    pub width: u32,
}
