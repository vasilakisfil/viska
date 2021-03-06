use common::{rsip, tokio::time::Instant};

#[derive(Debug)]
pub struct Errored {
    pub error: String,
    pub sip_message: Option<rsip::SipMessage>,
    pub entered_at: Instant,
}
