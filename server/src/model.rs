use serde::{Deserialize, Serialize};

// Response from com.atproto.server.createSession
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,
    pub did: String,
    email: String,
    #[serde(rename = "emailConfirmed")]
    email_confirmed: bool,
    handle: String,
    #[serde(rename = "refreshJwt")]
    refresh_jwt: String,
}

// Response from app.bsky.feed.getTimeline
#[derive(Serialize, Deserialize, Debug)]
pub struct Timeline {
    cursor: String,
    feed: Vec<Feed>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feed {
    post: Post,
    reason: Option<Reason>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    author: Author,
    cid: String,
    embed: Option<Embed>,
    #[serde(rename(deserialize = "indexedAt", serialize = "indexed_at"))]
    indexed_at: String,
    #[serde(rename(deserialize = "likeCount", serialize = "like_count"))]
    like_count: u32,
    record: Record,
    #[serde(rename(deserialize = "replyCount", serialize = "reply_count"))]
    reply_count: u32,
    #[serde(rename(deserialize = "repostCount", serialize = "repost_count"))]
    repost_count: u32,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    avatar: String,
    did: String,
    #[serde(rename(deserialize = "displayName", serialize = "display_name"))]
    display_name: String,
    handle: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename(deserialize = "$type", serialize = "record_type"))]
    record_type: String,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    created_at: String,
    text: String,
}

// Request to app.bsky.feed.post
#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePost {
    pub repo: String,
    pub collection: String,
    pub record: Record,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reason {
    #[serde(rename(deserialize = "$type", serialize = "record_type"))]
    pub record_type: String,
    pub by: By,
    #[serde(rename(deserialize = "indexedAt", serialize = "indexed_at"))]
    pub indexed_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct By {
    pub avatar: String,
    pub did: String,
    #[serde(rename(deserialize = "displayName", serialize = "display_name"))]
    pub display_name: String,
    pub handle: String,
    pub labels: Vec<String>,
    pub viewer: Viewer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Viewer {
    #[serde(rename(deserialize = "blockedBy", serialize = "blocked_by"))]
    pub blocked_by: bool,
    #[serde(rename(deserialize = "followedBy", serialize = "followed_by"))]
    pub followed_by: String,
    pub following: String,
    pub muted: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    #[serde(rename(deserialize = "$type", serialize = "type"))]
    pub type_: String,
    pub images: Option<Vec<Image>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub alt: String,
    #[serde(rename(deserialize = "aspectRatio", serialize = "aspect_ratio"))]
    pub aspect_ratio: AspectRatio,
    pub fullsize: String,
    pub thumb: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AspectRatio {
    pub height: u32,
    pub width: u32,
}