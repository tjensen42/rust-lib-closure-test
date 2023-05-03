use std::hint::black_box;

use bytes::Buf;
use rust_lib_closure_test::{deserialize_color_generic, deserialize_struct, Color};

pub fn main() {
    let buf: Vec<u8> = Vec::from([0x01, 0x02, 0x03]);

    // Call the generic function indirect (implemented in lib)
    let mut color = Color::default();
    let cursor = &mut buf.as_slice();
    deserialize_color_generic(black_box(cursor), &mut color);
    println!("color: {:?}", color);

    // Call the generic function direct
    let mut color = Color::default();
    let cursor = &mut buf.as_slice();
    deserialize_struct(black_box(cursor), &mut color, &DESER_COLOR_CLOSURE_BIN);
    println!("color: {:?}", color);
}

const DESER_COLOR_CLOSURE_BIN: [fn(&mut &[u8], &mut Color); 3] = [
    |r, s| s.r = r.get_u8(),
    |r, s| s.g = r.get_u8(),
    |r, s| s.b = r.get_u8(),
];
