use serde::{Deserialize, Serialize};

use crate::net::Msg;

#[derive(Debug)]
pub enum UdpFrameError {
    NoMsgFound,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum UdpMsgType {
    Request,  // Ask for some data
    Response, // Holds the data from a request
    Direct,   // Contains some latency-critic data
}

#[derive(Deserialize, Serialize, Debug, Clone)]
/// Encapsulates a message in a message that can be sent over UDP to ensure
/// ~~security (not for now)~~ and ease of flow control.
#[non_exhaustive]
pub struct UdpFrame {
    timestamp: i64,
    msg_type: UdpMsgType,
    msg: Option<Msg>,
}

impl UdpFrame {
    pub fn from(msg: &Msg, msg_type: UdpMsgType) -> Self {
        Self {
            timestamp: chrono::Utc::now().timestamp_millis(),
            msg_type,
            msg: Some(msg.clone()),
        }
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn get_msg(&self) -> Result<&Msg, UdpFrameError> {
        match &self.msg {
            Some(msg) => Ok(msg),
            None => Err(UdpFrameError::NoMsgFound),
        }
    }

    pub fn get_type(&self) -> UdpMsgType {
        self.msg_type.clone()
    }

    pub fn is_request(&self) -> bool {
        self.msg_type == UdpMsgType::Request
    }

    pub fn is_response(&self) -> bool {
        self.msg_type == UdpMsgType::Response
    }

    pub fn is_direct(&self) -> bool {
        self.msg_type == UdpMsgType::Direct
    }

    pub fn from_bytes(buf: &[u8]) -> Self {
        // Cleanup the messages with innecessary `\0` character
        let message = buf
            .into_iter()
            .map(|x| *x)
            .take_while(|&x| x != 0)
            .collect::<Vec<_>>();
        ron::de::from_bytes(&message).unwrap()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        ron::to_string(&self.msg).unwrap().into_bytes()
    }
}
