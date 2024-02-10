use serde::{Serialize, Deserialize};

// Response from com.atproto.server.createSession
#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    #[serde(rename = "accessJwt")]
    access_jwt: String,
    did: String,
    email: String,
    #[serde(rename = "emailConfirmed")]
    email_confirmed: bool,
    handle: String,
    #[serde(rename = "refreshJwt")]
    refresh_jwt: String,
}
