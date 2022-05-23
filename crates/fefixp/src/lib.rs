//! *FIX Performance Session Layer*
//! ([FIXP](https://www.fixtrading.org/standards/fixp-online/)) support.

type SessionId = u128;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlowType {
    Recoverable,
    Idempotent,
    Unsequenced,
    None,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum MessageType {
    Sequence,
    Context,
    MessageTemplate,
    Negotiate,
}

#[derive(Debug, Clone)]
pub struct Sequence {
    pub next_seq_number: u64,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub session_id: SessionId,
    pub next_seq_number: u64,
}

#[derive(Debug, Clone)]
pub struct MessageTemplate {
    pub encoding_type: u32,
    pub effective_time: u64,
    pub version: Vec<u8>,
    pub template: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Negotiate {
    pub session_id: SessionId,
    pub client_flow: FlowType,
    pub credentials: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct NegotiationReject {
    pub session_id: SessionId,
    pub reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Establish {
    pub session_id: SessionId,
    pub next_seq_number: u64,
    pub credentials: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct EstablishmentAck {
    pub next_seq_number: u64,
}
