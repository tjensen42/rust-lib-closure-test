use bytes::Buf;

#[derive(Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub enum Error {
    NotEnoughBytes,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn deserialize_u8(reader: &mut &[u8], v: &mut u8) -> Result<()> {
    if reader.remaining() < core::mem::size_of::<u8>() {
        return Err(Error::NotEnoughBytes);
    }
    *v = reader.get_u8();
    Ok(())
}

///
/// RESULT
///

pub fn deserialize_struct_lib_result<S, F>(
    reader: &mut &[u8],
    data: &mut S,
    func: &[F; 3],
) -> Result<()>
where
    F: Fn(&mut &[u8], &mut S) -> Result<()>,
{
    if reader.remaining() < 3 {
        return Err(Error::NotEnoughBytes);
    }
    func[0](reader, data)?;
    func[1](reader, data)?;
    func[2](reader, data)?;
    Ok(())
}

#[inline(never)]
pub fn deserialize_color_generic_lib_result(reader: &mut &[u8], color: &mut Color) -> Result<()> {
    deserialize_struct_lib_result(reader, color, &DESER_COLOR_CLOSURE_LIB_RESULT)
}

const DESER_COLOR_CLOSURE_LIB_RESULT: [fn(&mut &[u8], &mut Color) -> Result<()>; 3] = [
    |r, s| deserialize_u8(r, &mut s.r),
    |r, s| deserialize_u8(r, &mut s.g),
    |r, s| deserialize_u8(r, &mut s.b),
];

///
/// PLAIN
///

pub fn deserialize_struct_lib_plain<S, F>(reader: &mut &[u8], data: &mut S, func: &[F; 3])
where
    F: Fn(&mut &[u8], &mut S),
{
    if reader.remaining() >= 3 {
        func[0](reader, data);
        func[1](reader, data);
        func[2](reader, data);
    }
}

#[inline(never)]
pub fn deserialize_color_generic_lib_plain(reader: &mut &[u8], color: &mut Color) {
    deserialize_struct_lib_plain(reader, color, &DESER_COLOR_CLOSURE_LIB_PLAIN);
}

const DESER_COLOR_CLOSURE_LIB_PLAIN: [fn(&mut &[u8], &mut Color); 3] = [
    |r, s| s.r = r.get_u8(),
    |r, s| s.g = r.get_u8(),
    |r, s| s.b = r.get_u8(),
];
