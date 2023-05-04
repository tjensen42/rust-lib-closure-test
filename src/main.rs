use std::hint::black_box;

use bytes::Buf;
use rust_lib_closure_test::{
    deserialize_color_generic_lib_plain, deserialize_color_generic_lib_result,
    deserialize_struct_lib_plain, Result,
};
use rust_lib_closure_test::{deserialize_struct_lib_result, deserialize_u8, Color};

pub fn main() {
    let buf: Vec<u8> = Vec::from([0x01, 0x02, 0x03]);

    // LIB RESULT
    let mut color = Color::default();
    let cursor = &mut buf.as_slice();
    deserialize_color_generic_lib_result(black_box(cursor), &mut color).unwrap();
    println!("color: {:?}", color);

    // MAIN RESULT
    let mut color = Color::default();
    let cursor = &mut buf.as_slice();
    deserialize_color_generic_main_result(black_box(cursor), &mut color).unwrap();
    println!("color: {:?}", color);

    // LIB PLAIN
    let mut color = Color::default();
    let cursor = &mut buf.as_slice();
    deserialize_color_generic_lib_plain(black_box(cursor), &mut color);
    println!("color: {:?}", color);

    // MAIN PLAIN
    let mut color = Color::default();
    let cursor = &mut buf.as_slice();
    deserialize_color_generic_main_plain(black_box(cursor), &mut color);
    println!("color: {:?}", color);
}

///
/// RESULT
///

const DESER_COLOR_CLOSURE_MAIN_RESULT: [fn(&mut &[u8], &mut Color) -> Result<()>; 3] = [
    |r, s| deserialize_u8(r, &mut s.r),
    |r, s| deserialize_u8(r, &mut s.g),
    |r, s| deserialize_u8(r, &mut s.b),
];

#[inline(never)]
fn deserialize_color_generic_main_result(reader: &mut &[u8], color: &mut Color) -> Result<()> {
    deserialize_struct_lib_result(reader, color, &DESER_COLOR_CLOSURE_MAIN_RESULT)
}

///
/// PLAIN
///

const DESER_COLOR_CLOSURE_MAIN_PLAIN: [fn(&mut &[u8], &mut Color); 3] = [
    |r, s| s.r = r.get_u8(),
    |r, s| s.g = r.get_u8(),
    |r, s| s.b = r.get_u8(),
];

#[inline(never)]
fn deserialize_color_generic_main_plain(reader: &mut &[u8], color: &mut Color) {
    deserialize_struct_lib_plain(reader, color, &DESER_COLOR_CLOSURE_MAIN_PLAIN);
}
