
// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

// v = 1024 + 255+ 63 + 255
/*
1_024
Decimal
_ is just for readability
_ as a visual separator (ignored by the compiler)
Value = 1024

0xff
Hexadecimal (0x)
ff = 15 × 16 + 15
Value = 255

0o77
Octal (0o)
77 = 7 × 8 + 7
Value = 63

0b1111_1111
Binary (0b)
_ is again just visual
11111111₂
Value = 255
*/