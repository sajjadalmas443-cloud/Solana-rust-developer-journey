use std::collections::HashMap;

fn main() {
    // 1. Vector: Resizable List (Aisa array jo chhota/bada ho sakta hai)
    let mut solana_accounts: Vec<String> = Vec::new();
    
    // Items add karna
    solana_accounts.push(String::from("Account_A"));
    solana_accounts.push(String::from("Account_B"));
    
    println!("Pehla account: {}", solana_accounts[0]);
    println!("Total accounts: {}", solana_accounts.len());

    // 2. HashMap: Key-Value Pair Data (Dictionary jaisa)
    let mut balances: HashMap<String, u64> = HashMap::new();
    
    // Data insert karna
    balances.insert(String::from("Sajjad"), 500);
    balances.insert(String::from("Ali"), 250);

    // Value read karna
    if let Some(balance) = balances.get("Sajjad") {
        println!("Sajjad ka balance hai: {} SOL", balance);
    }
}