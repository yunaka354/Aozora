use serde::{Deserialize, Serialize};

// Response from com.atproto.server.createSession
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    #[serde(rename = "accessJwt")]
    pub access_jwt: String,
    did: String,
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
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    author: Author,
    cid: String,
    #[serde(rename(deserialize="indexedAt", serialize="indexed_at"))]
    indexed_at: String,
    #[serde(rename(deserialize="likeCount", serialize="like_count"))]
    like_count: u32,
    record: Record,
    #[serde(rename(deserialize="replyCount", serialize="reply_count"))]
    reply_count: u32,
    #[serde(rename(deserialize="repostCount", serialize="repost_count"))]
    repost_count: u32,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    avatar: String,
    did: String,
    #[serde(rename(deserialize="displayName", serialize="display_name"))]
    display_name: String,
    handle: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename(deserialize="$type", serialize="record_type"))]
    record_type: String,
    #[serde(rename(deserialize="createdAt", serialize="created_at"))]
    created_at: String,
    text: String,
}
