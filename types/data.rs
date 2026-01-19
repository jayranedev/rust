//integers:
//Signed: i
//Unsigned: u

//8 bits:
let x: i8
let x: u8

//16 bits:
let x: i16
let x: u16

//32 bits:
let x: i32
let x: u32

//64 bits:
let x: i64
let x: u64

//128 bits:
let x: i128
let x: u128

//arch: architecture: means dependent on computers architecture
//example: if a computer is 64bits arch then it will be 64bits
//64bits ~ 8 bytes
let x: isize
let x: usize


//Default types in rust:
// integers: i32
// float: f64

// what is a word???
// The processor does not read one byte at a time from memory
//but reads one word at a time

// in a 32-bit processor it can accress 4 bytes at a time
// in a 64-bit processor it can accress 8 bytes at a time

// when a var is mutable it can only take similar data type
//example u128 will only take u128 and give error if i128 is passed



// Fill the blank
fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

// so here the variable v is of type u16, and the value is of type u8
// we cannot make a variable take a value of diff type
// so we change the type of the variable by using as
// 38_u8 as u16; The _u8 syntax is a literal suffix that tells the compiler to treat the number 38 specifically as a 1-byte unsigned integer. 

