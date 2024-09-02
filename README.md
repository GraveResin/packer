# Packer

This is just a basic packing thing I made that packs multiples of both signed and unsigned values into larger values. I decided not to make packing of floats since the f8 isn't in rust and f128 is unstable. I don't know if this will be useful for anyone or if this is even made well. still learning the language and programming in general. :]

Also if there is a mistake in this readme or if it could be better then you all know what to do.

You can store up to four i8 values into a single i32 value and the same for the unsigned counterpart. The packs range from as small as two i8 values into a single i16, to sixteen i8 values into a single i128.

## Example:

- This is for packing two i8 values into an i16. 
```rust
pub fn packi8toi16(a: i8, b: i8) -> i16 {
    let mut pack: i16 = 0;
    let values = [a, b];

    for (i, &value) in values.iter().enumerate() {
        let data_shift = i * 8;
        pack |= (value as i16 & 0xFF) << data_shift;
    }
    pack
}
```

- You call the function and just pass the values to have them packed into an i16 value or other pack function for different value types like so:

```rust
fn main() {
    let number0: i8 = 5;
    let number1: i8 = 10;
    packed_data = packi8toi16(number0, number1);
    println!("number0 is {}\nnumber1 is {}", (packed_data & 0xFF) as i8, ((packed_data >> 8) & 0xFF) as i8);
}
```

- This will pack the data into a sinlge i16 value then you can pull the data out like I did in the print line or you can pull the packed data back out using the unpack function of the data types you packed like so.

```rust
pub fn unpacku8tou16(pack: u16) -> (u8, u8) {
    let mut values = [0u8; 2];

    for i in 0..2 {
        let data_shift = i * 8;
        values[i] = (pack >> data_shift) as u8;
    }
    (values[0], values[1])
}
```

- This will bring the data back out and require them to be new variables but can make it easy to modify the individual values and then repack them if you don't want to modify them the hard way.



```rust
fn main() {
    let number0: i8 = 5;
    let number1: i8 = 10;
    pack_data = packi8toi16(number0, number1);
    let (new_number0, new_number1) = unpacku8tou32(pack_data);

}
```




