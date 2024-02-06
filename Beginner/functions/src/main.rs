
fn main(){   //1  
    let result = add_Numbers(2, 3);
    println!("{}", result);
}


fn add_Numbers(a: i32, b: i32) -> i32{   
    return a + b;
}