/* 
fn main() {
    let s1 = String::from("Rust"); // heap allocated string
    let r1 = &s1;    // Reference are defined with 

    println!("s1 is: {s1}");
}

fn print_string(p1: &String) {
    println!("{p1}");
} 
*/

/*

fn main() {
    let mut s1 = String::from("Rust"); // heap allocated string
    let r1 = &s1;    // Reference are defined with &
    print_string(r1);

    let r2 = &mut s1;
    add_to_string(r2);
    println!("s1 is: {s1}");
}

fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome!");
}

fn print_string(p1: &String) {
    println!("{p1}");
}

*/


fn main() {
    let mut s1 = String::from("Rust"); // heap allocated string
    let r1 = &s1;    // Reference are defined with &
    print_string(r1);

    let r2 = &mut s1;
    add_to_string(r2);
    println!("s1 is: {s1}");
    let s2 = generate_string();
}

fn generate_string() -> String {
    String::from("Ferris")
}

fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome!");
}

fn print_string(p1: &String) {
    println!("{p1}");
}