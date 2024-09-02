#![allow(dead_code)]

/*
////////////////////////////////////////////////////////////////////////////////////////////
 
This is the same as signed_packs just without the masks to keep the signed thingamabobs.

////////////////////////////////////////////////////////////////////////////////////////////
*/

////////////////////////////////(u128 packs)////////////////////////////////
////////////(u64)////////////
pub fn packu64tou128(a: u64, b: u64) -> u128 {
    let mut pack: u128 = 0;
    let values = [a, b];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 64;
        pack |= (value as u128) << data_shift;
    }
    pack
}

pub fn unpacku64tou128(pack: u128) -> (u64, u64) {
    let mut values = [0u64; 2];

    for i in 0..2 {
        let data_shift = i * 64;
        values[i] = (pack >> data_shift) as u64;
    }
    (values[0], values[1])
}
////////////(===)////////////

////////////(u32)////////////
pub fn packu32tou128(a: u32, b: u32, c: u32, d: u32) -> u128 {
    let mut pack: u128 = 0;
    let values = [a, b, c, d];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 32;
        pack |= (value as u128) << data_shift;
    }
    pack
}

pub fn unpacku32tou128(pack: u128) -> (u32, u32, u32, u32) {
    let mut values = [0u32; 4];

    for i in 0..4 {
        let data_shift = i * 32;
        values[i] = (pack >> data_shift) as u32;
    }
    (values[0], values[1], values[2], values[3])
}
////////////(===)////////////

////////////(u16)////////////
pub fn packu16tou128(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> u128 {
    let mut pack: u128 = 0;
    let values = [a, b, c, d, e, f, g, h];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 16;
        pack |= (value as u128) << data_shift;
    }
    pack
}

pub fn unpacku16tou128(pack: u128) -> (u16, u16, u16, u16, u16, u16, u16, u16) {
    let mut values = [0u16; 8];

    for i in 0..8 {
        let data_shift = i * 16;
        values[i] = (pack >> data_shift) as u16;
    }
    (values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7])
}
////////////(===)////////////

////////////(u8)////////////
pub fn packu8tou128(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8, i: u8, j: u8, k: u8, l: u8, m: u8, n: u8, o: u8, p: u8) -> u128 {
    let mut pack: u128 = 0;
    let values = [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 8;
        pack |= (value as u128) << data_shift;
    }
    pack
}

pub fn unpacku8tou128(pack: u128) -> (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) {
    let mut values = [0u8; 16];

    for i in 0..4 {
        let data_shift = i * 8;
        values[i] = (pack >> data_shift) as u8;
    }
    (values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7], values[8], values[9], values[10], values[11], values[12], values[13], values[14], values[15])
}
////////////(==)////////////
////////////////////////////////(==========)////////////////////////////////

////////////////////////////////(u64 packs)////////////////////////////////
////////////(u32)////////////
pub fn packu32tou64(a: u32, b: u32) -> u64 {
    let mut pack: u64 = 0;
    let values = [a, b];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 32;
        pack |= (value as u64) << data_shift;
    }
    pack
}

pub fn unpacku32tou64(pack: u64) -> (u32, u32) {
    let mut values = [0u32; 2];

    for i in 0..2 {
        let data_shift = i * 32;
        values[i] = (pack >> data_shift) as u32;
    }
    (values[0], values[1])
}
////////////(===)////////////

////////////(u16)////////////
pub fn packu16tou64(a: u16, b: u16, c: u16, d: u16) -> u64 {
    let mut pack: u64 = 0;
    let values = [a, b, c, d];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 16;
        pack |= (value as u64) << data_shift;
    }
    pack
}

pub fn unpacku16tou64(pack: u64) -> (u16, u16, u16, u16) {
    let mut values = [0u16; 4];

    for i in 0..4 {
        let data_shift = i * 16;
        values[i] = (pack >> data_shift) as u16;
    }
    (values[0], values[1], values[2], values[3])
}
////////////(===)////////////

////////////(u8)////////////
pub fn packu8tou64(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u64 {
    let mut pack: u64 = 0;
    let values = [a, b, c, d, e, f, g, h];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 8;
        pack |= (value as u64) << data_shift;
    }
    pack
}

pub fn unpacku8tou64(pack: u64) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    let mut values = [0u8; 8];

    for i in 0..8 {
        let data_shift = i * 8;
        values[i] = (pack >> data_shift) as u8;
    }
    (values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7])
}
////////////(==)////////////
////////////////////////////////(=========)////////////////////////////////

////////////////////////////////(u32 packs)////////////////////////////////
////////////(u16)////////////
pub fn packu16to132(a: u16, b: u16) -> u32 {
    let mut pack: u32 = 0;
    let values = [a, b];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 16;
        pack |= (value as u32) << data_shift;
    }
    pack
}

pub fn unpacku6tou32(pack: u32) -> (u16, u16) {
    let mut values = [0u16; 2];

    for i in 0..2 {
        let data_shift = i * 16;
        values[i] = (pack >> data_shift) as u16;
    }
    (values[0], values[1])
}
////////////(===)////////////

////////////(u8)////////////
pub fn packu8tou32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    //todo!("This for loop use works and must be used in all pack functions");
    let mut pack: u32 = 0;
    let values = [a, b, c, d];

    for (i, &value)  in values.iter().enumerate() {
        let data_shift = i * 8;
        pack |= (value as u32) << data_shift;
    }
    pack
}
pub fn unpacku8tou32(pack: u32) -> (u8, u8, u8, u8) {
    let mut values = [0u8; 4]; //array to store the unpacked u8 values

    for i in 0..4 {
        let data_shift = i * 8; //takes the i value and multiplies it by 8 which is data type.
        values[i] = (pack >> data_shift) as u8; //extracts the byte and casts it to u8
    }
    (values[0], values[1], values[2], values[3]) //returns unpacked values as a tuple
}
////////////(==)////////////
////////////////////////////////(=========)////////////////////////////////

////////////////////////////////(u16 packs)////////////////////////////////
////////////(u8)////////////
pub fn packu8tou16(a: u8, b: u8) -> u16 {
    let mut pack: u16 = 0;
    let values = [a, b];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 8;
        pack |= (value as u16) << data_shift;
    }
    pack
}

pub fn unpacku8tou16(pack: u16) -> (u8, u8) {
    let mut values = [0u8; 2];

    for i in 0..2 {
        let data_shift = i * 8;
        values[i] = (pack >> data_shift) as u8;
    }
    (values[0], values[1])
}
////////////(==)////////////
////////////////////////////////(=========)////////////////////////////////
