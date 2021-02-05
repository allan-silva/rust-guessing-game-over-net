#[cfg(test)]
mod tests;

extern crate serde;
extern crate serde_json;
extern crate byteorder;



use std::convert::{From, TryFrom};

use serde::{Serialize, Deserialize};

use byteorder::{NetworkEndian, WriteBytesExt};

pub const FRAME_END: u8 = 0x4;

#[derive(Debug, PartialEq)]
pub struct ProtocolHeader {
    header: String,
}

impl Default for ProtocolHeader {
    fn default() -> Self {
        ProtocolHeader {
            header: String::from("GG010"),
        }
    }
}

impl TryFrom<Vec<u8>> for ProtocolHeader {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match String::from_utf8(value) {
            Ok(incoming_header) => {
                if ProtocolHeader::default().header == incoming_header {
                    Ok(ProtocolHeader::default())
                } else {
                    Err(ProtocolHeader::default().header)
                }
            }
            _ => Err(ProtocolHeader::default().header),
        }
    }
}

struct Frame {
    class: u8,
    method: u8,
    payload: Vec<u8>,
}

impl Frame {
    fn new(class: u8, method: u8, payload: Vec<u8>) -> Self {
        Frame {
            class,
            method,
            payload
        }
    }
}

impl From<Frame> for Vec<u8> {
    fn from(frame: Frame) -> Vec<u8> {
        let mut frame_bytes = Vec::new();
        frame_bytes.write_u8(frame.class).unwrap();
        frame_bytes.write_u8(frame.method).unwrap();
        frame_bytes.extend(&frame.payload);
        frame_bytes.write_u8(FRAME_END).unwrap();
        frame_bytes
    }
}

impl TryFrom<Vec<u8>> for Frame {
    type Error = String;
    
    fn try_from(mut payload: Vec<u8>) -> Result<Self, String> {
        let frame_end = payload.pop();

        if frame_end.is_none() || frame_end.unwrap() != FRAME_END {
            return Err(String::from("Malformed frame"))
        }

        Ok(Frame {
            class: 1,
            method: 1,
            payload: Vec::new()
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ConnectionConstraints {
    max_name_size: u8
}

impl Default for ConnectionConstraints {
    fn default() -> Self {
        ConnectionConstraints {
            max_name_size: 10u8
        }
    }
}

impl From<ConnectionConstraints> for Vec<u8> {
    fn from(connection_constraints: ConnectionConstraints) -> Vec<u8> {
        let json = serde_json::to_string(&connection_constraints).unwrap();
        json.as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::ProtocolHeader;
    use std::convert::TryFrom;

    #[test]
    fn test_protocol_header_try_from() {
        let header = "GG010".as_bytes().to_vec();
        let protocol_header = ProtocolHeader::try_from(header);
        assert!(
            protocol_header.is_ok(),
            "Error converting GG010 to ProtocolHeader"
        );
        assert_eq!(ProtocolHeader::default(), protocol_header.unwrap());
    }

    #[test]
    fn test_protocol_header_try_from_error() {
        let header = "GG011".as_bytes().to_vec();
        let protocol_header = ProtocolHeader::try_from(header);
        assert!(
            protocol_header.is_err(),
            "Invalid value converted to ProtocolHeader"
        );
        assert_eq!(
            ProtocolHeader::default().header,
            protocol_header.unwrap_err()
        );
    }
}
