mod signed_packs;
mod unsigned_packs;
use signed_packs::{packi8toi16, packi8toi32, unpacki8toi32, packi64toi128, unpacki64toi128};
use unsigned_packs::{packu64tou128, packu8tou32, unpacku8tou32};

//old and to be removed
/*fn packu32(a: u8, b: u8, c: u8) -> u32 {
    (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) //| ((d as u32) << 24)
}*/

/*//////////////////////////////////////////////////////////////////
This is just me messing round with data types and packing them. I do not know if it will be useful to anyone. I am not an expert.
*///////////////////////////////////////////////////////////////////

struct Math {
    number0: u8,
    number1: u8,
    multi: u8,
    total: u8,
    pack: u32,
}

struct Math2 {
    number0: i8,
    number1: i8,
    multi: i8,
    total: i8,
    pack: i32,
}

struct Math3 {
    number0: i8,
    number1: i8,
    total: i8,
    pack: i16,
}

fn main() {
    let mut _data = Math {
        number0: 20,
        number1: 10,
        multi: 2,
        total: 0,
        pack: 0,
    };

    let mut math2 = Math2 {
        number0: -2,
        number1: 2,
        multi: -4,
        total: 0,
        pack: 0,
    };

    let mut math3 = Math3 {
        number0: 10,
        number1: -20,
        total: 0,
        pack: 0,
    };

    //in the event that you don't need all of the values filled at the time, just put a 0 to satisfy it.
    _data.pack = packu8tou32(_data.number0, _data.number1, _data.multi, _data.total);

    let test64 = packu64tou128(10, 20);
    println!("This is just to test the packing of two 64 bit values into an i128. the first value should be 10. The second, 20");
    println!(
        "The first 64 bit test values is: {}\nThe second is {}\n//////(end test)//////",
        test64 as u64,
        (test64 >> 64) as u64
    );
    //takes one and two from _data and adds them together it also has data packing and unpacking of up to 4 u8 values into a single u32

    //these names (first, second, multi, and total) are just some quick dummy names i came up with. Hope they are not that confusing.
    let (first, second, multi, mut total) = unpacku8tou32(_data.pack);
    println!("Total was: {}", total);
    total = first + second;

    println!(
        "first, {}, is added to second, {}.\nThe total added is: {}\n",
        first, second, total
    );
    total = total * multi;
    println!(
        "multi, {}, is what the total is multiplied by leading the new total to be, {}\n",
        multi, total
    );

    //This is me testing is I can extract a bit of the u32
    //let testpack: u8 = ((_data.pack >> 8) & 0xFF) as u8;
    println!("The testpack is: {}", (_data.pack >> 8) as u8);
    println!("The packed data is: {:b}", _data.pack);

    math2.total = math2.number0 + math2.number1;
    math2.pack = packi8toi32(math2.number0, math2.number1, math2.multi, math2.total);
    println!("\n\n\nThis is to test if the signed values are properly saved");
    println!("Packed data {:b}", math2.pack);
    println!(
        "number0 is {}.\n number1 is {}.\n total is {}.\n multi is {}",
        math2.number0, math2.number1, math2.total, math2.multi
    );

    println!(
        "The packed data for multi is {}",
        ((math2.pack >> 16) & 0xFF) as i8
    );

    println!("\nNote that I am aware that all the print statments are messy.");

    for i in 0..4 {
        let shift = i * 8;
        let extracted = ((math2.pack >> shift) & 0xFF) as i8;
        println!(
            "Extracted i8 value {}: {:#x} ({})",
            i + 1,
            extracted,
            extracted
        );
    }
    ///////////////////////////////////////////////////////////////
    math2.pack = packi8toi32(math2.number0, math2.number1, math2.multi, math2.total);
    println!("\n\nThis is to test packing and reading the pack with for loops. This is the i8 to i32 pack");
    for i in 0..4 {
        let data_shift = i * 8;
        let extract = ((math2.pack >> data_shift) & 0xFF) as i8;
        println!("{}: {:#x} ({})", i + 1, extract, extract);
    }
    let (number0, number1, multi, total) = unpacki8toi32(math2.pack);
    println!("This to test if the for loop for unpacking works correctly.\n");
    println!("{}, {}, {}, {}", number0, number1, multi, total);

    ///////////////////////////////////////////////////////////////
    math3.pack = packi8toi16(math3.number0, math3.number1);
    println! {"\n\nThis is testing the i8 to i16 with for loops {}.", math3.total};
    for i in 0..2 {
        let data_shift = i * 8;
        let extract = ((math3.pack >> data_shift) & 0xFF) as i8;
        print!("{}: {:#x} ({})\n", i + 1, extract, extract);
    }

    /////////////////
    let x: i64 = 50;
    let y: i64 = -10;
    let packed = packi64toi128(x, y);
    println!("\n\n\n{}, {}", packed as i64, ((packed >> 64) & 0xFFFFFFFFFFFFFFFF) as i64);

    let (unpack1, unpack2) = unpacki64toi128(packed);
    print!("{}, {}\n", unpack1, unpack2);
    


}
