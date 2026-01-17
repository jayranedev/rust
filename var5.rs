// define_x function so we move the statement that needs x in the function scope

fn main(){
    define_x();
}

fn define_x(){
    let x: &str = "Hello";
    println!("{} World!", x);
}
