use crate::driver::parser::{parse_frame_header, parse_registration_method, parse_registration_ok_method};
use crate::protocol::{Class, ConnectionConstraints, Method, Registration};
use byteorder::{ByteOrder, NetworkEndian};
use serde_json;

#[test]
fn test_parse_frame_header() {
    let frame_header_bytes: [u8; 5] = [1, 2, 42, 42, 42];
    let (remain_bytes, frame_header) = parse_frame_header(&frame_header_bytes).unwrap();
    assert_eq!(Class::Connection, frame_header.class);
    assert_eq!(Method::RegistrationOk, frame_header.method);
    assert_eq!(3, remain_bytes.len());
}

#[test]
fn test_parse_registration() {
    let constraints = ConnectionConstraints::default();
    let constraints_bytes = serde_json::to_string(&constraints)
        .unwrap()
        .as_bytes()
        .to_owned();

    let mut payload: Vec<u8> = Vec::new();

    let mut constraits_len_bytes: [u8; 4] = [0; 4];
    NetworkEndian::write_u32(&mut constraits_len_bytes, constraints_bytes.len() as u32);

    payload.extend(&constraits_len_bytes);
    payload.extend(&constraints_bytes);

    let (remain_bytes, registration) = parse_registration_method(&payload).unwrap();
    assert_eq!(0, remain_bytes.len());
    assert_eq!(constraints_bytes.len(), registration.size as usize);
    assert_eq!(constraints.max_name_size, registration.constraints.max_name_size);
}

#[test]
fn test_parse_registration_ok() {
    let user_name = String::from("Allançõí");
    let user_name_bytes = user_name.as_bytes().to_owned();
    let user_name_size = user_name_bytes.len() as u8;
    let mut payload = Vec::new();
    payload.push(user_name_size);
    payload.extend(&user_name_bytes);

    let (remain_bytes, registration_ok) = parse_registration_ok_method(&payload).unwrap();
    assert_eq!(0, remain_bytes.len());
    assert_eq!(user_name_bytes.len(), registration_ok.size as usize);
    assert_eq!(user_name, registration_ok.user_name);
}