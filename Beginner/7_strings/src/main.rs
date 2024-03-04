/* 
fn main(){
    let s1 = "Anmol";
    let s2 = String::from("Anmol");
    let s3 = "Anmol".to_string();
    let s4 = "Anmol".to_owned();
    let s5 =&s4[..];
    println!("{}", s5);
}
*/

/* 

fn main(){
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    s.replace_range(.., "baz"); //(range: .., replace_with: "baz");
    println!("{}", s);
}

*/

//concatination 

/* 
fn main(){
    // + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 +&s2;

    println!("{}", s3);
}
*/

fn main(){
    //format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s)
}