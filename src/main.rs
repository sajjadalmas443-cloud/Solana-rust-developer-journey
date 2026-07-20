// 1. Result Enum: Success (Ok) ya Failure (Err) handle karne ke liye
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Zero se divide nahi ho sakta!"))
    } else {
        Ok(numerator / denominator)
    }
}

// 2. Option Enum: Something (Some) ya Nothing (None) ke liye
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Sajjad"))
    } else {
        None
    }
}

fn main() {
    // Result Test
    match divide(10.0, 2.0) {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => println!("Error aaya: {}", e),
    }

    // Option Test
    match find_user(1) {
        Some(name) => println!("User mila: {}", name),
        None => println!("User nahi mila!"),
    }
}