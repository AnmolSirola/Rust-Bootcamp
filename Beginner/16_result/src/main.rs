fn main() {
    let username = get_username(1);
    if let Some(name) = username {
        println!("{name}");
    }
}

fn get_username(user_id: u32) -> Option<String> {
    let query =
        format!("GET username FROM users WHERE id={user_id}");
    let db_result = query_db(query);
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query string is empty!"))
    } else {
        Ok(String::from("Ferris"))
    }
}

/*
// Define a function that returns a Result
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        // Return an error if the divisor is zero
        Err("Cannot divide by zero")
    } else {
        // Return the result if the division is successful
        Ok(a / b)
    }
}

fn main() {
    // Example usage of the divide function
    match divide(10, 2) {
        Ok(result) => {
            println!("Result: {}", result);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    // Example with an error case
    match divide(8, 2) {
        Ok(result) => {
            println!("Result: {}", result);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
 */