fn main() {
    let s1 = String::from("Rust"); // heap allocated string
    let s2 = s1.clone();
    print_string(s1.clone()); //4
    let s3 = generate_string(); //3
    let s4 = add_to_string(s2); //2

    println!("s1 is: {s1}");
    println!("s3 is: {s3}");
    println!("s4 is: {s4}");

    let x = 10;
    let y = x;
    print_integer(x); //1
    println!("x is: {x}");
}

fn print_integer(i: i32) { 
    println!("i is: {i}");
}

fn add_to_string(mut p1: String) -> String {  //functions taking ownership and giving it back
    p1.push_str(" is awesome!");
    p1
}

fn generate_string() -> String {  //ownership out of the function
    String::from("Ferris")
}

fn print_string(p1: String) {  //ownership into the funcion
    println!("{p1}");
} // p1 is dropped