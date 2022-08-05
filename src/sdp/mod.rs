use combine::error::StringStreamError;
use thiserror::Error;

mod sdp;
pub use sdp::{Fingerprint, MediaLine, MediaType, Mid, Sdp, SessionId};

mod parser;
pub(crate) use parser::candidate as parse_candidate;

#[derive(Debug, Error)]
pub enum SdpError {
    #[error("SDP parse: {0}")]
    Parse(#[from] StringStreamError),

    #[error("SDP inconsistent: {0}")]
    Inconsistent(String),
}