use bytes::Buf;

#[derive(Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// Deserialize some struct with 3 Fields
#[inline(never)]
pub fn deserialize_struct<S, F>(reader: &mut &[u8], data: &mut S, func: &[F; 3])
where
    F: Fn(&mut &[u8], &mut S),
{
    if reader.remaining() >= 3 {
        func[0](reader, data);
        func[1](reader, data);
        func[2](reader, data);
    }
}

// Closure array to deserialize a Color struct
pub const DESER_COLOR_CLOSURE_LIB: [fn(&mut &[u8], &mut Color); 3] = [
    |r, s| s.r = r.get_u8(),
    |r, s| s.g = r.get_u8(),
    |r, s| s.b = r.get_u8(),
];

// Implement the generic function in the library
pub fn deserialize_color_generic(reader: &mut &[u8], color: &mut Color) {
    deserialize_struct(reader, color, &DESER_COLOR_CLOSURE_LIB)
}
