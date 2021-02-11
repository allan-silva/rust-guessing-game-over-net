use std::str;

use crate::protocol::{
    Class, ConnectionConstraints, FrameHeader, Method, Registration, RegistrationOk, User, UserRegistred
};

use byteorder::{ByteOrder, NetworkEndian};

use serde_json;

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
    take(size)(input).map(|(i, o)| (i, serde_json::from_slice(o).unwrap()))
}

pub fn parse_registration_method(input: &[u8]) -> Result<(&[u8], Registration), String> {
    let constraints_size_parse: IResult<&[u8], u32> =
        take(4u8)(input).map(|(i, o)| (i, NetworkEndian::read_u32(o)));
    let (remain_bytes, constraint_size) = constraints_size_parse.unwrap();
    let (remain_bytes, connection_constraints) =
        parse_registration_constraints(remain_bytes, constraint_size as usize).unwrap();
    Ok((
        remain_bytes,
        Registration::new(constraint_size, connection_constraints),
    ))
}

fn parse_user_name(input: &[u8], size: usize) -> IResult<&[u8], String> {
    take(size)(input).map(|(i, o)| (i, str::from_utf8(o).unwrap().to_string()))
}

pub fn parse_registration_ok_method(input: &[u8]) -> Result<(&[u8], RegistrationOk), String> {
    let user_name_size_parse: IResult<&[u8], u8> = take(1u8)(input).map(|(i, o)| (i, o[0]));
    let (remain_bytes, user_name_size) = user_name_size_parse.unwrap();
    let (remain_bytes, user_name) = parse_user_name(remain_bytes, user_name_size as usize).unwrap();
    Ok((remain_bytes, RegistrationOk::new(user_name_size, user_name)))
}

fn parse_user(input: &[u8], size: usize) -> IResult<&[u8], User> {
    take(size)(input).map(|(i, o)| {
        (
            i,
            serde_json::from_slice(o).unwrap()
        )
    })
}

pub fn parse_user_registred_method(input: &[u8]) -> Result<(&[u8], UserRegistred), String> {
    let user_size_parse: IResult<&[u8], u32> = take(4u8)(input).map(|(i, o)| {
        (
            i,
            NetworkEndian::read_u32(o)
        )
    });

    let (remain_bytes, user_size) = user_size_parse.unwrap();
    let (remain_bytes, user) = parse_user(remain_bytes, user_size as usize).unwrap();
    Ok((remain_bytes, UserRegistred::new(user_size, user)))
}
