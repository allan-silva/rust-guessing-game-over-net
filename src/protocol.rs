extern crate serde;
extern crate serde_json;
extern crate byteorder;

use std::sync::mpsc::Sender;
use crate::messages::ServerCommand;
use std::io::{BufReader, BufRead};
use std::convert::{From, TryFrom};
use std::net::{TcpStream, Shutdown};
use std::io::{Write, Read};

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

pub struct Connection {
    stream: TcpStream,
    main_tx: Sender<ServerCommand>
}

impl Connection {
    pub fn new(stream: TcpStream, main_tx: Sender<ServerCommand>) -> Self {
        Connection {
            stream,
            main_tx
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut reader = BufReader::new(&self.stream);
        let mut buffer_protocol_header = [0; 5];
        reader.read_exact(&mut buffer_protocol_header).unwrap();

        match ProtocolHeader::try_from(buffer_protocol_header.to_vec()) {
            Ok(_) => self.registration(),
            Err(protocol_header) => {
                self.reply_header(protocol_header);
                self.close_connection();
                Err(String::from("Connection closed - Invalid header"))
            }
        }
    }

    pub fn registration(&mut self) -> Result<(), String> {
        let constraints = ConnectionConstraints::default();
        let constraints = Vec::<u8>::from(constraints);
        
        let mut registration_payload = Vec::new();

        let bytes_length = constraints.len() as u32;

        registration_payload.write_u32::<NetworkEndian>(bytes_length).unwrap();
        registration_payload.extend(constraints);

        let frame = Frame::new(1, 1, registration_payload);
        let frame = &Vec::<u8>::from(frame);

        match self.stream.write(frame) {
            Ok(_) => self.registration_ok(),
            Err(error) => Err(format!("{}", error))
        }
    }

    pub fn registration_ok(&mut self) -> Result<(), String> {
        loop {
            let mut reader = BufReader::new(&self.stream);
            let mut frame = Vec::new();
            reader.read_until(FRAME_END, &mut frame).unwrap();
        }
    }

    fn reply_header(&mut self, protocol_header: String) {
        self.stream.write(protocol_header.as_bytes()).unwrap();
    }

    fn close_connection(&mut self) {
        self.stream.flush().unwrap();
        self.stream.shutdown(Shutdown::Both).unwrap();
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
