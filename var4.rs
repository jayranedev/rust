//scope

// fn main(){
//     let x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
//     }
//     println!{"The value of x is {} and value of y is {}", x, y};
// }
// the above code will give error as the var y is out of scope as it is the part of fucntion
//we added y to global so ...


fn main(){
    let x: i32 = 10;
    let y: i32 = 5;
    {        
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!{"The value of x is {} and value of y is {}", x, y};
}