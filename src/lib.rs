#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Marker {
    Null = b'Z',
    Noop = b'N',
    True = b'T',
    False = b'F',
    Int8 = b'i',
    Uint8 = b'U',
    Int16 = b'I',
    Int32 = b'l',
    Int64 = b'L',
    Float32 = b'd',
    Float64 = b'D',
    HighPrecisionNumber = b'H',
    Char = b'C',
    String = b'S',
    ArrayStart = b'[',
    ArrayEnd = b']',
    ObjectStart = b'{',
    ObjectEnd = b'}',
}
