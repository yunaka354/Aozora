use serde::{Serialize, Deserialize};

// Response from com.atproto.server.createSession
#[derive(Serialize, Deserialize, Debug)]
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
    feed: Vec<Feed>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feed {
    post: Post
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    author: Author,
    cid: String,
    #[serde(rename = "indexedAt")]
    indexed_at: String,
    #[serde(rename = "likeCount")]
    like_count: u32,
    record: Record,
    #[serde(rename = "replyCount")]
    reply_count: u32,
    #[serde(rename = "repostCount")]
    repost_count: u32,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    avatar: String,
    did: String,
    #[serde(rename = "displayName")]
    display_name: String,
    handle: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename = "$type")]
    record_type: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    text: String,
}