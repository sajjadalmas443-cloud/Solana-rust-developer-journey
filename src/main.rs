fn main() {
    // Function ko call kar rahe hain
    say_hello();
    
    // Values pass karke add function call kar rahe hain
    let sum = add(15, 25);
    println!("Dono numbers ka sum hai: {}", sum);
}

// Chhota sa custom function
fn say_hello() {
    println!("Solana Rust Developer Journey!");
}

// Parameters lene wala aur value return karne wala function
fn add(a: i32, b: i32) -> i32 {
    a + b // Rust mein aakhri line par semicolon (;) na lagayein to wo return ho jata hai
}