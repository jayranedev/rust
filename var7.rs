//destructuring a tuple with mutable variable

// fn main(){
//     let (x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
//     println!("success!");
// }

// this will give error as var in rust are by default immutable
// there are two ways to solve this:
//shadowing:
// fn main(){
//     let (x, y) = (1, 2);
//     let x = x + 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
//     println!("success!");
// }

//Mutability:
fn main(){
    let (mut x, y) = (1, 2); // so here x is mutable
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("success!");
}
