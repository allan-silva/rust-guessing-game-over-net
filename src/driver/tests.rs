use crate::driver::parser::{parse_frame_header, parse_registration_method};
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

    let mut frame_bytes: Vec<u8> = Vec::new();

    let mut constraits_len_bytes: [u8; 4] = [0; 4];
    NetworkEndian::write_u32(&mut constraits_len_bytes, constraints_bytes.len() as u32);

    frame_bytes.extend(&constraits_len_bytes);
    frame_bytes.extend(&constraints_bytes);

    let (remain_bytes, registration) = parse_registration_method(&frame_bytes).unwrap();
    assert_eq!(0, remain_bytes.len());
    println!("{},{}", constraints_bytes.len(), registration.size);
    assert_eq!(constraints_bytes.len(), registration.size as usize);
    assert_eq!(constraints.max_name_size, registration.constraints.max_name_size);
}
