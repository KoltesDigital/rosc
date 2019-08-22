extern crate byteorder;
extern crate rosc_supercollider;

use byteorder::{BigEndian, ByteOrder};
use std::mem;

use rosc_supercollider::{decoder, encoder};
use rosc_supercollider::{OscAddress, OscError, OscPacket, OscType};

#[test]
fn test_decode_string_address_no_args() {
    // message to build: /some/valid/address/4 ,
    let raw_addr = "/some/valid/address/4";
    let addr = encoder::encode_string(raw_addr);
    let type_tags = encoder::encode_string(",");
    let merged: Vec<u8> = addr.into_iter().chain(type_tags.into_iter()).collect();
    let osc_packet: Result<OscPacket, OscError> = decoder::decode(&merged);
    assert!(osc_packet.is_ok());
    match osc_packet {
        Ok(OscPacket::Message(msg)) => {
            assert_eq!(OscAddress::String(raw_addr.to_string()), msg.addr);
            assert!(msg.args.is_none());
        }
        Ok(_) => panic!("Expected an OscMessage!"),
        Err(e) => panic!(e),
    }
}

#[test]
fn test_decode_string_address_args() {
    // /another/valid/address/123 ,fdih 3.1415 3.14159265359 12345678i32
    // -1234567891011
    let addr = encoder::encode_string("/another/valid/address/123");
    // args
    let f = 3.1415f32;
    let mut f_bytes: [u8; 4] = [0u8; 4];
    BigEndian::write_f32(&mut f_bytes, f);
    assert_eq!(BigEndian::read_f32(&f_bytes), f);

    let d = 3.14159265359f64;
    let mut d_bytes: [u8; 8] = [0u8; 8];
    BigEndian::write_f64(&mut d_bytes, d);
    assert_eq!(BigEndian::read_f64(&d_bytes), d);

    let i = 12345678i32;
    let i_bytes: [u8; 4] = unsafe { mem::transmute(i.to_be()) };

    let l = -1234567891011i64;
    let h_bytes: [u8; 8] = unsafe { mem::transmute(l.to_be()) };

    let blob_size: [u8; 4] = unsafe { mem::transmute(6u32.to_be()) };
    let blob: Vec<u8> = vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8];

    let s = "I am an osc test string.";
    assert!(s.is_ascii());
    // Osc strings are null terminated like in C!
    let s_bytes: Vec<u8> = encoder::encode_string(s);

    let c = '$';
    let c_bytes: [u8; 4] = unsafe { mem::transmute((c as u32).to_be()) };

    let a = vec![OscType::Int(i), OscType::Float(f), OscType::Int(i)];

    let type_tags = encoder::encode_string(",fdsTFibhNIc[ifi]");

    let args: Vec<u8> = f_bytes
        .iter()
        .chain(d_bytes.iter())
        .chain(s_bytes.iter())
        .chain(i_bytes.iter())
        .chain(blob_size.iter())
        .chain(blob.iter())
        .chain(vec![0u8, 0u8].iter())
        .chain(h_bytes.iter())
        .chain(c_bytes.iter())
        // array content
        .chain(i_bytes.iter())
        .chain(f_bytes.iter())
        .chain(i_bytes.iter())
        .map(|x| *x)
        .collect::<Vec<u8>>();

    let merged: Vec<u8> = addr
        .into_iter()
        .chain(type_tags.into_iter())
        .chain(args)
        .collect::<Vec<u8>>();

    match decoder::decode(&merged).unwrap() {
        OscPacket::Message(msg) => {
            for arg in msg.args.unwrap() {
                match arg {
                    OscType::Int(x) => assert_eq!(i, x),
                    OscType::Long(x) => assert_eq!(l, x),
                    OscType::Float(x) => assert_eq!(f, x),
                    OscType::Double(x) => assert_eq!(d, x),
                    OscType::String(x) => assert_eq!(s, x),
                    OscType::Blob(x) => assert_eq!(blob, x),
                    // cant assign bool args to type_tag
                    // , so there is no real test wether the value is
                    // correct or not
                    OscType::Bool(_) => (),
                    OscType::Inf => (),
                    OscType::Nil => (),
                    // test time-tags, midi-messages and chars
                    OscType::Char(x) => assert_eq!(c, x),
                    OscType::Array(x) => assert_eq!(a, x.content),
                    _ => panic!(),
                }
            }
        }
        _ => panic!("Expected an OSC message!"),
    }
}

#[test]
fn test_decode_int_address_no_args() {
    // \x00\x00\x00\x2a,
    let addr: u32 = 42;
    let type_tags = encoder::encode_string(",");
    let merged: Vec<u8> = vec![0, 0, 0, addr as u8]
        .into_iter()
        .chain(type_tags.into_iter())
        .collect();
    let osc_packet: Result<OscPacket, OscError> = decoder::decode(&merged);
    assert!(osc_packet.is_ok());
    match osc_packet {
        Ok(OscPacket::Message(msg)) => {
            assert_eq!(OscAddress::Int(addr), msg.addr);
            assert!(msg.args.is_none());
        }
        Ok(_) => panic!("Expected an OscMessage!"),
        Err(e) => panic!(e),
    }
}

#[test]
fn test_decode_int_address_args() {
    // \x00\x00\x00\x2a,fdih 3.1415 3.14159265359 12345678i32
    // -1234567891011
    let addr: u32 = 42;
    // args
    let f = 3.1415f32;
    let mut f_bytes: [u8; 4] = [0u8; 4];
    BigEndian::write_f32(&mut f_bytes, f);
    assert_eq!(BigEndian::read_f32(&f_bytes), f);

    let d = 3.14159265359f64;
    let mut d_bytes: [u8; 8] = [0u8; 8];
    BigEndian::write_f64(&mut d_bytes, d);
    assert_eq!(BigEndian::read_f64(&d_bytes), d);

    let i = 12345678i32;
    let i_bytes: [u8; 4] = unsafe { mem::transmute(i.to_be()) };

    let l = -1234567891011i64;
    let h_bytes: [u8; 8] = unsafe { mem::transmute(l.to_be()) };

    let blob_size: [u8; 4] = unsafe { mem::transmute(6u32.to_be()) };
    let blob: Vec<u8> = vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8];

    let s = "I am an osc test string.";
    assert!(s.is_ascii());
    // Osc strings are null terminated like in C!
    let s_bytes: Vec<u8> = encoder::encode_string(s);

    let c = '$';
    let c_bytes: [u8; 4] = unsafe { mem::transmute((c as u32).to_be()) };

    let a = vec![OscType::Int(i), OscType::Float(f), OscType::Int(i)];

    let type_tags = encoder::encode_string(",fdsTFibhNIc[ifi]");

    let args: Vec<u8> = f_bytes
        .iter()
        .chain(d_bytes.iter())
        .chain(s_bytes.iter())
        .chain(i_bytes.iter())
        .chain(blob_size.iter())
        .chain(blob.iter())
        .chain(vec![0u8, 0u8].iter())
        .chain(h_bytes.iter())
        .chain(c_bytes.iter())
        // array content
        .chain(i_bytes.iter())
        .chain(f_bytes.iter())
        .chain(i_bytes.iter())
        .map(|x| *x)
        .collect::<Vec<u8>>();

    let merged: Vec<u8> = vec![0, 0, 0, addr as u8]
        .into_iter()
        .chain(type_tags.into_iter())
        .chain(args)
        .collect::<Vec<u8>>();

    match decoder::decode(&merged).unwrap() {
        OscPacket::Message(msg) => {
            for arg in msg.args.unwrap() {
                match arg {
                    OscType::Int(x) => assert_eq!(i, x),
                    OscType::Long(x) => assert_eq!(l, x),
                    OscType::Float(x) => assert_eq!(f, x),
                    OscType::Double(x) => assert_eq!(d, x),
                    OscType::String(x) => assert_eq!(s, x),
                    OscType::Blob(x) => assert_eq!(blob, x),
                    // cant assign bool args to type_tag
                    // , so there is no real test wether the value is
                    // correct or not
                    OscType::Bool(_) => (),
                    OscType::Inf => (),
                    OscType::Nil => (),
                    // test time-tags, midi-messages and chars
                    OscType::Char(x) => assert_eq!(c, x),
                    OscType::Array(x) => assert_eq!(a, x.content),
                    _ => panic!(),
                }
            }
        }
        _ => panic!("Expected an OSC message!"),
    }
}
