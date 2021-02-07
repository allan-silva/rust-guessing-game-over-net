use crate::protocol::{Class, ConnectionConstraints, FrameHeader, Method, Registration};

use byteorder::{NetworkEndian, ReadBytesExt, ByteOrder};

use serde_json;

use std::io::{Read};

use nom::bytes::complete::take;
use nom::IResult;

fn parse_frame_class(input: &[u8]) -> IResult<&[u8], Class> {
    take(1usize)(input).map(|(i, o)| {
        (
            i,
            match o {
                [1] => Class::Connection,
                _ => Class::Unknow,
            },
        )
    })
}

fn parse_frame_method(input: &[u8]) -> IResult<&[u8], Method> {
    take(1u8)(input).map(|(i, o)| {
        (
            i,
            match o {
                [1] => Method::Registration,
                [2] => Method::RegistrationOk,
                [3] => Method::User,
                _ => Method::Unknow,
            },
        )
    })
}

pub fn parse_frame_header(input: &[u8]) -> Result<(&[u8], FrameHeader), String> {
    let (remain_bytes, class) = parse_frame_class(input).unwrap();
    let (remain_bytes, method) = parse_frame_method(remain_bytes).unwrap();
    Ok((remain_bytes, FrameHeader::new(class, method)))
}

fn parse_registration_constraints(
    input: &[u8],
    size: usize,
) -> IResult<&[u8], ConnectionConstraints> {
    take(size)(input).map(|(i, o)| {
        (
            i,
            serde_json::from_slice(o).unwrap()
        )
    })
}

pub fn parse_registration_method(input: &[u8]) -> Result<(&[u8], Registration), String> {
    let constraints_size_parse: IResult<&[u8], u32> = take(4u8 as usize)(input).map(|(i, o)| {
        (
            i,
            NetworkEndian::read_u32(&o)
        )
    });
    let (remain_bytes, constraint_size) = constraints_size_parse.unwrap();
    println!("SIZE: {}", constraint_size);
    let (remain_bytes, connection_constraints) = parse_registration_constraints(remain_bytes, constraint_size as usize).unwrap();
    Ok((remain_bytes, Registration::new(constraint_size, connection_constraints)))
}
