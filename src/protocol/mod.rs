#[cfg(test)]
mod tests;

extern crate byteorder;
extern crate serde;
extern crate serde_json;

use std::convert::{From, TryFrom};

use serde::{Deserialize, Serialize};

const FRAME_END: u8 = 0x4;

#[derive(Debug, PartialEq)]
pub enum Class {
    Connection,
    Unknow,
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Registration,
    RegistrationOk,
    User,
    Unknow,
}

#[derive(Debug)]
pub struct FrameHeader {
    pub class: Class,
    pub method: Method,
}

impl FrameHeader {
    pub fn new(class: Class, method: Method) -> Self {
        FrameHeader { class, method }
    }
}

pub trait FramePayload {
    type Payload;
    fn get(self) -> Self::Payload;
}

pub struct Frame<T> {
    pub header: FrameHeader,
    pub payload: Box<dyn FramePayload<Payload = T>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionConstraints {
    pub max_name_size: u8,
}

impl Default for ConnectionConstraints {
    fn default() -> Self {
        ConnectionConstraints {
            max_name_size: 10u8,
        }
    }
}

#[derive(Debug)]
pub struct Registration {
    pub size: u32,
    pub constraints: ConnectionConstraints,
}

impl Registration {
    pub fn new(size: u32, constraints: ConnectionConstraints) -> Self {
        Registration { size, constraints }
    }
}

impl FramePayload for Registration {
    type Payload = Registration;

    fn get(self) -> Self {
        self
    }
}

#[derive(Debug)]
pub struct RegistrationOk {
    pub size: u8,
    pub user_name: String,
}

impl RegistrationOk {
    pub fn new(size: u8, user_name: String) -> Self {
        RegistrationOk { size, user_name }
    }
}

impl FramePayload for RegistrationOk {
    type Payload = RegistrationOk;

    fn get(self) -> Self {
        self
    }
}

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
