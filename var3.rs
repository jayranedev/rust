fn main(){
    let mut x = 5; //Use mut to mark a variable as mutable
    // By default a variable is immutable
    x += 2;

    assert_eq!(x, 7);
    println!("Success!");
}