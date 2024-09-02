#![allow(dead_code)]

/*
////////////////////////////////////////////////////////////////////////////////////////////
this is just me testing out packing multiple smaller values into a single larger value.
If only 3 values are needed then just have the unused empty value be zero... at least for the moment...maybe... :]
////////////////////////////////////////////////////////////////////////////////////////////
*/

////////////////////////////////(i128 packs)////////////////////////////////
////////////(i64)////////////
pub fn packi64toi128(a: i64, b: i64) -> i128 {
    let mut pack: i128 = 0;
    let values = [a, b];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 64;
        pack |= (value as i128 & 0xFFFFFFFFFFFFFFFF) << data_shift;
    }
    pack
}

pub fn unpacki64toi128(pack: i128) -> (i64, i64) {
    let mut values = [0i64; 2];

    for i in 0..2 {
        let data_shift = i * 64;
        values[i] = ((pack >> data_shift) & 0xFFFFFFFFFFFFFFFF) as i64;
    }
    (values[0], values[1])
}
////////////(===)////////////

////////////(i32)////////////
pub fn packi32toi128(a: i32, b: i32, c: i32, d: i32) -> i128 {
    let mut pack: i128 = 0;
    let values = [a, b, c, d];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 32;
        pack |= (value as i128 & 0xFFFFFFFF) << data_shift;
    }
    pack
}

pub fn unpacki32toi128(pack: i128) -> (i32, i32, i32, i32) {
    let mut values = [0i32; 4];

    for i in 0..4 {
        let data_shift = i * 32;
        values[i] = ((pack >> data_shift) & 0xFFFFFFFF) as i32;
    }
    (values[0], values[1], values[2], values[3])
}
////////////(===)////////////

////////////(i16)////////////
pub fn packi16toi128(a: i16, b: i16, c: i16, d: i16, e: i16, f: i16, g: i16, h: i16) -> i128 {
    let mut pack: i128 = 0;
    let values = [a, b, c, d, e, f, g, h];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 16;
        pack |= (value as i128 & 0xFFFF) << data_shift;
    }
    pack
}

pub fn unpacki16toi128(pack: i128) -> (i16, i16, i16, i16, i16, i16, i16, i16) {
    let mut values = [0i16; 8];

    for i in 0..8 {
        let data_shift = i * 16;
        values[i] = ((pack >> data_shift) & 0xFFFF) as i16;
    }
    (values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7])
}
////////////(===)////////////

////////////(i8)////////////
pub fn packi8toi128(a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8, h: i8, i: i8, j: i8, k: i8, l: i8, m: i8, n: i8, o: i8, p: i8) -> i128 {
    let mut pack: i128 = 0;
    let values = [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 8;
        pack |= (value as i128 & 0xFF) << data_shift;
    }
    pack
}

pub fn unpacki8toi128(pack: i128) -> (i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) {
    let mut values = [0i8; 16];

    for i in 0..4 {
        let data_shift = i * 8;
        values[i] = ((pack >> data_shift) & 0xFF) as i8;
    }
    (values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7], values[8], values[9], values[10], values[11], values[12], values[13], values[14], values[15])
}
////////////(==)////////////
////////////////////////////////(==========)////////////////////////////////

////////////////////////////////(i64 packs)////////////////////////////////
////////////(i32)////////////
pub fn packi32toi64(a: i32, b: i32) -> i64 {
    let mut pack: i64 = 0;
    let values = [a, b];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 32;
        pack |= (value as i64 & 0xFFFFFFFF) << data_shift;
    }
    pack
}

pub fn unpacki32toi64(pack: i64) -> (i32, i32) {
    let mut values = [0i32; 2];

    for i in 0..2 {
        let data_shift = i * 32;
        values[i] = ((pack >> data_shift) & 0xFFFFFFFF) as i32;
    }
    (values[0], values[1])
}
////////////(===)////////////

////////////(i16)////////////
pub fn packi16toi64(a: i16, b: i16, c: i16, d: i16) -> i64 {
    let mut pack: i64 = 0;
    let values = [a, b, c, d];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 16;
        pack |= (value as i64 & 0xFFFF) << data_shift;
    }
    pack
}

pub fn unpacki16toi64(pack: i64) -> (i16, i16, i16, i16) {
    let mut values = [0i16; 4];

    for i in 0..4 {
        let data_shift = i * 16;
        values[i] = ((pack >> data_shift) & 0xFFFF) as i16;
    }
    (values[0], values[1], values[2], values[3])
}
////////////(===)////////////

////////////(i8)////////////
pub fn packi8toi64(a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8, h: i8) -> i64 {
    let mut pack: i64 = 0;
    let values = [a, b, c, d, e, f, g, h];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 8;
        pack |= (value as i64 & 0xFF) << data_shift;
    }
    pack
}

pub fn unpacki8toi64(pack: i64) -> (i8, i8, i8, i8, i8, i8, i8, i8) {
    let mut values = [0i8; 8];

    for i in 0..8 {
        let data_shift = i * 8;
        values[i] = ((pack >> data_shift) & 0xFF) as i8;
    }
    (values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7])
}
////////////(==)////////////
////////////////////////////////(=========)////////////////////////////////

////////////////////////////////(i32 packs)////////////////////////////////
////////////(i16)////////////
pub fn packi16to132(a: i16, b: i16) -> i32 {
    let mut pack: i32 = 0;
    let values = [a, b];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 16;
        pack |= (value as i32 & 0xFFFF) << data_shift;
    }
    pack
}

pub fn unpacki6toi32(pack: i32) -> (i16, i16) {
    let mut values = [0i16; 2];

    for i in 0..2 {
        let data_shift = i * 16;
        values[i] = ((pack >> data_shift) & 0xFFFF) as i16;
    }
    (values[0], values[1])
}
////////////(===)////////////

////////////(i8)////////////
pub fn packi8toi32(a: i8, b: i8, c: i8, d: i8) -> i32 {
    //todo!("This for loop use works and must be used in all pack functions");
    let mut pack: i32 = 0;
    let values = [a, b, c, d];

    for (i, &value)  in values.iter().enumerate() {
        let data_shift = i * 8;
        pack |= (value as i32 & 0xFF) << data_shift;
    }
    pack
}
pub fn unpacki8toi32(pack: i32) -> (i8, i8, i8, i8) {
    let mut values = [0i8; 4]; //array to store the unpacked i8 values

    for i in 0..4 {
        let data_shift = i * 8; //takes the i value and multiplies it by 8 which is data type.
        values[i] = ((pack >> data_shift) & 0xFF) as i8; //extracts the byte and casts it to i8
    }
    (values[0], values[1], values[2], values[3]) //returns unpacked values as a tuple
}
////////////(==)////////////
////////////////////////////////(=========)////////////////////////////////

////////////////////////////////(i16 packs)////////////////////////////////
////////////(i8)////////////
pub fn packi8toi16(a: i8, b: i8) -> i16 {
    let mut pack: i16 = 0;
    let values = [a, b];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 8;
        pack |= (value as i16 & 0xFF) << data_shift;
    }
    pack
}

pub fn unpacki8toi16(pack: i16) -> (i8, i8) {
    let mut values = [0i8; 2];

    for i in 0..2 {
        let data_shift = i * 8;
        values[i] = ((pack >> data_shift) & 0xFF) as i8;
    }
    (values[0], values[1])
}
////////////(==)////////////
////////////////////////////////(=========)////////////////////////////////
