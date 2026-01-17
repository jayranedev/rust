fn main(){
    let x: i32 = 5;
    let _y: i32; // underscore helps avoid the warning

    assert_eq!(x, 5);
    println!("Success!");
}

//Or to get rid of the unused variable we can use the following way:
#[allow(unused_variables)]
fn main(){
    let x = 1;
}