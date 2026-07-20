fn main() {
    // 1. Ownership Transfer (Move)
    let s1 = String::from("Solana");
    let s2 = s1; // 's1' ka owner ab 's2' ban gaya. s1 ab valid nahi raha!

    // println!("{}", s1); // Agar isay uncomment karenge to Rust compile error dega!
    println!("s2 ki value hai: {}", s2);

    // 2. Borrowing / Reference (&)
    let name = String::from("Sajjad");
    print_length(&name); // & lagane se ownership transfer nahi hoti, sirf read access milta hai
    
    println!("Main abhi bhi '{}' ko yahan use kar sakta hoon!", name);
}

fn print_length(s: &String) {
    println!("String ki length hai: {}", s.len());
}