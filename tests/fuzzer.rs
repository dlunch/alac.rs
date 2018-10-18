extern crate alac;

const COOKIE_A: &'static [u8] = b"\x00\x00\x10\x00\x00\x10\x28\x0a\x0e\x02\x00\xff\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xac\x44\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2c\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x0c\x00\x68\xc5\x06\x00\x01\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\xef\xcd\xab\x89\xff\xff\xff\xff\x40\xd3\x06\x00\x01\x00\x00\x00\xb0\x02\x20\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x22\x00\x00\x00\x07\x06\x00\x00\x00\x00\x00\x00\x00\x00\x00\x20\x21\x43\x50\x58\x05\x00\x00\x00\x00\xf0\x00\x00\x04\x00\x00\x00\x00\x00\x00";

#[allow(non_snake_case)]
#[test]
fn ID_FIL_skip_bytes_underflow() {
    let data = b"\xde\x00\x0a\xff";
    assert_decode_err(COOKIE_A, data);
}

#[test]
fn lpc_quant_zero() {
    let data = b"\x00\x00\x10\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00";
    assert_decode_err(COOKIE_A, data);
}

#[test]
fn lpc_predict_lpc_coefs_overflow() {
    let data = b"\x00\x00\x10\x00\x00\x00\x09\x98\x00\x15\x05\x00\x00\x00\x00\x00\x20";
    assert_decode_err(COOKIE_A, data);
}

#[test]
fn negative_bit_depth() {
    let data = b"\x04\x00\x18\x00\x00\x00\x04\x00\x10\x15\x02\x00\x00\x10\x00\x00\x00\x09";
    assert_decode_err(COOKIE_A, data);
}

fn assert_decode_err(cookie: &[u8], data: &[u8]) {
    let stream_info = alac::StreamInfo::from_cookie(cookie).expect("error reading cookie");
    let mut decoder = alac::Decoder::new(stream_info);
    let mut out = vec![0; decoder.stream_info().max_samples_per_packet() as usize];
    assert!(decoder.decode_packet(data, &mut out).is_err());
}
