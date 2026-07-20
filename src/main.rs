// 1. Struct (Custom Data Type)
struct UserAccount {
    username: String,
    balance: u64,
    is_active: bool,
}

// 2. Enum (Options/States define karne ke liye)
enum AccountStatus {
    Active,
    Frozen,
    Closed,
}

fn main() {
    // Struct ka instance banana
    let user = UserAccount {
        username: String::from("Sajjad"),
        balance: 100,
        is_active: true,
    };

    println!("User: {}", user.username);
    println!("Balance: {} SOL", user.balance);

    // Enum value set karna
    let status = AccountStatus::Active;

    // Pattern Matching (Match Statement)
    match status {
        AccountStatus::Active => println!("Account status: Active hai!"),
        AccountStatus::Frozen => println!("Account status: Frozen hai!"),
        AccountStatus::Closed => println!("Account status: Closed hai!"),
    }
}