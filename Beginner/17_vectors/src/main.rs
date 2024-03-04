fn main() {
    // Creating an empty vector of strings
    let mut v = Vec::new();

    // Pushing strings onto the vector
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    // Creating a vector with integers
    let v2 = vec![1, 2, 3];

    // Accessing the first element using indexing (can panic if out of bounds)
    let s = &v[0];

    // Alternatively, use get method which returns an Option<&T>
    let s = v.get(0);

    // Checking if the element exists before printing
    if let Some(e) = s {
        println!("{e}");
    }

    // Iterating over mutable references to each element and modify them
    for s in &mut v {
        s.push_str("!");
    }

    // Iterating over immutable references to each element and print them
    for s in &v {
        println!("{s}");
    }

    // Creating an empty vector v3
    let mut v3 = vec![];

    // Move all elements from v to v3 using into_iter
    for s in v.into_iter() {
        v3.push(s);
    }

    // Attempting to access v after moving its elements would result in a compilation error
    // Uncommenting the line below will result in an error
    // let i = v.get(0);
}
