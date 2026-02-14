# Practical Exercises & Learning Guide

## Getting Started

### Exercise 1: Understanding Account Creation
**Goal**: Learn how accounts are created and initialized

```rust
// Your task: Create 3 accounts with different types
let bank = BankingSystem::new(100_000); // ₦1,000 minimum balance

// 1. Create a Savings account for "John" with ₦50,000
let john_savings = bank.create_account(
    "JOHN001".to_string(),
    AccountType::Savings,
    5_000_000, // Remember: amounts in kobo!
    "NGN".to_string(),
).unwrap();

// 2. Create a Current account for "Jane" with ₦100,000
// TODO: Fill this in

// 3. Create a Fixed account for "Bob" with ₦200,000
// TODO: Fill this in

// Question: Why do we use i64 instead of f64 for money?
// Answer: To avoid floating-point rounding errors in financial calculations
```

### Exercise 2: Understanding Minor Units
**Goal**: Practice converting between Naira and Kobo

```rust
// Conversion Examples:
// ₦1.00 = 100 kobo
// ₦1,000.00 = 100,000 kobo
// ₦0.50 = 50 kobo

// Practice conversions:
fn naira_to_kobo(naira: f64) -> i64 {
    (naira * 100.0) as i64
}

fn kobo_to_naira(kobo: i64) -> f64 {
    kobo as f64 / 100.0
}

// Test yourself:
assert_eq!(naira_to_kobo(50.0), 5_000);      // ₦50.00
assert_eq!(naira_to_kobo(1_234.56), 123_456); // ₦1,234.56
assert_eq!(kobo_to_naira(100), 1.0);         // 100 kobo = ₦1.00

// Why is this important?
// Example: 0.1 + 0.2 in floating point ≠ 0.3 exactly
// But: 10 + 20 in integers = 30 exactly (always!)
```

## Core Concepts Exercises

### Exercise 3: Simple Deposits
**Goal**: Understand single-leg transactions

```rust
let bank = BankingSystem::new(100_000);
let account = bank.create_account(
    "CUST001".to_string(),
    AccountType::Savings,
    500_000, // ₦5,000
    "NGN".to_string(),
).unwrap();

// Make 3 deposits
bank.deposit(&account.account_number, 100_000, "Deposit 1".to_string()).unwrap();
bank.deposit(&account.account_number, 250_000, "Deposit 2".to_string()).unwrap();
bank.deposit(&account.account_number, 150_000, "Deposit 3".to_string()).unwrap();

// Question: What should the final balance be?
// Initial: ₦5,000
// + ₦1,000 + ₦2,500 + ₦1,500 = ?
// TODO: Calculate and verify

let final_balance = bank.get_balance(&account.account_number).unwrap();
println!("Final balance: ₦{:.2}", final_balance as f64 / 100.0);
```

### Exercise 4: Withdrawal Validation
**Goal**: Learn about business rule validation

```rust
let bank = BankingSystem::new(100_000); // Minimum balance: ₦1,000
let account = bank.create_account(
    "CUST001".to_string(),
    AccountType::Current,
    300_000, // ₦3,000
    "NGN".to_string(),
).unwrap();

// Scenario 1: Valid withdrawal
match bank.withdraw(&account.account_number, 100_000, "ATM".to_string()) {
    Ok(_) => println!("✓ Withdrawal successful"),
    Err(e) => println!("✗ {}", e),
}

// Scenario 2: Try to withdraw too much
// This should fail because: 3,000 - 2,500 = 500 < 1,000 (minimum)
match bank.withdraw(&account.account_number, 250_000, "Large withdrawal".to_string()) {
    Ok(_) => println!("✓ Withdrawal successful"),
    Err(e) => println!("✗ {}", e), // This should print an error
}

// Question: What's the maximum you can withdraw?
// Balance: ₦3,000
// Minimum: ₦1,000
// Available: ₦2,000
// After first withdrawal: ₦2,000
// Maximum now: ₦1,000
```

### Exercise 5: Understanding Transfers (Double-Entry)
**Goal**: Learn how double-entry bookkeeping works

```rust
let bank = BankingSystem::new(100_000);

let alice = bank.create_account("ALICE".to_string(), AccountType::Current, 
                                 1_000_000, "NGN".to_string()).unwrap();
let bob = bank.create_account("BOB".to_string(), AccountType::Savings,
                               500_000, "NGN".to_string()).unwrap();

println!("BEFORE TRANSFER:");
println!("Alice: ₦{:.2}", bank.get_balance(&alice.account_number).unwrap() as f64 / 100.0);
println!("Bob:   ₦{:.2}", bank.get_balance(&bob.account_number).unwrap() as f64 / 100.0);

// Transfer ₦3,000 from Alice to Bob
bank.transfer(&alice.account_number, &bob.account_number, 
              300_000, "Gift".to_string()).unwrap();

println!("\nAFTER TRANSFER:");
println!("Alice: ₦{:.2}", bank.get_balance(&alice.account_number).unwrap() as f64 / 100.0);
println!("Bob:   ₦{:.2}", bank.get_balance(&bob.account_number).unwrap() as f64 / 100.0);

// IMPORTANT: Verify the system balance hasn't changed!
// Before: 10,000 + 5,000 = 15,000
// After:  7,000  + 8,000 = 15,000 ✓
// This is the essence of double-entry bookkeeping!
```

## Advanced Exercises

### Exercise 6: Error Handling
**Goal**: Learn to handle different error types

```rust
fn process_customer_request(bank: &BankingSystem, 
                            from: &str, 
                            to: &str, 
                            amount: i64) {
    match bank.transfer(from, to, amount, "Transfer".to_string()) {
        Ok(txn) => {
            println!("✓ Transfer successful!");
            println!("  Transaction ID: {}", txn.transaction_id);
            println!("  Reference: {}", txn.reference);
        },
        Err(BankingError::InsufficientFunds(msg)) => {
            println!("✗ Transaction declined: {}", msg);
            println!("  Suggested action: Check balance or reduce amount");
        },
        Err(BankingError::AccountNotFound(account)) => {
            println!("✗ Invalid account: {}", account);
            println!("  Suggested action: Verify account number");
        },
        Err(BankingError::AccountInactive(msg)) => {
            println!("✗ Account inactive: {}", msg);
            println!("  Suggested action: Contact customer service");
        },
        Err(e) => {
            println!("✗ Transaction failed: {}", e);
        }
    }
}

// Test with different scenarios
// TODO: Create scenarios that trigger each error type
```

### Exercise 7: Transaction History Analysis
**Goal**: Learn to query and analyze transaction data

```rust
fn analyze_account_activity(bank: &BankingSystem, account_number: &str) {
    let history = bank.get_transaction_history(account_number);
    
    println!("Transaction Analysis for {}", account_number);
    println!("─────────────────────────────────────");
    
    // Count transactions by type
    let deposits = history.iter()
        .filter(|t| t.transaction_type == TransactionType::Deposit)
        .count();
    
    let withdrawals = history.iter()
        .filter(|t| t.transaction_type == TransactionType::Withdrawal)
        .count();
    
    let transfers_in = history.iter()
        .filter(|t| t.transaction_type == TransactionType::Transfer && 
                     t.to_account.as_ref() == Some(&account_number.to_string()))
        .count();
    
    let transfers_out = history.iter()
        .filter(|t| t.transaction_type == TransactionType::Transfer && 
                     t.from_account.as_ref() == Some(&account_number.to_string()))
        .count();
    
    println!("Deposits:        {}", deposits);
    println!("Withdrawals:     {}", withdrawals);
    println!("Transfers In:    {}", transfers_in);
    println!("Transfers Out:   {}", transfers_out);
    println!("Total:           {}", history.len());
    
    // Calculate total money in/out
    let mut total_in = 0i64;
    let mut total_out = 0i64;
    
    for txn in &history {
        match txn.transaction_type {
            TransactionType::Deposit => total_in += txn.amount,
            TransactionType::Withdrawal => total_out += txn.amount,
            TransactionType::Transfer => {
                if txn.to_account.as_ref() == Some(&account_number.to_string()) {
                    total_in += txn.amount;
                } else {
                    total_out += txn.amount;
                }
            },
            _ => {}
        }
    }
    
    println!("\nTotal In:  ₦{:.2}", total_in as f64 / 100.0);
    println!("Total Out: ₦{:.2}", total_out as f64 / 100.0);
    println!("Net:       ₦{:.2}", (total_in - total_out) as f64 / 100.0);
}
```

### Exercise 8: Concurrent Transactions (Advanced)
**Goal**: Understand thread safety and race conditions

```rust
use std::sync::Arc;
use std::thread;

fn demonstrate_thread_safety() {
    // Shared banking system across threads
    let bank = Arc::new(BankingSystem::new(100_000));
    
    // Create account with ₦10,000
    let account = bank.create_account(
        "CONCURRENT_TEST".to_string(),
        AccountType::Current,
        1_000_000,
        "NGN".to_string(),
    ).unwrap();
    
    let account_num = account.account_number.clone();
    
    println!("Initial balance: ₦{:.2}", account.balance_as_decimal());
    println!("Starting 10 concurrent withdrawals of ₦500 each...\n");
    
    let mut handles = vec![];
    
    // Spawn 10 threads, each withdrawing ₦500
    for i in 0..10 {
        let bank_clone = Arc::clone(&bank);
        let acc = account_num.clone();
        
        let handle = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(i * 10)); // Stagger starts
            
            match bank_clone.withdraw(&acc, 50_000, format!("Withdrawal {}", i)) {
                Ok(_) => println!("Thread {}: ✓ Withdrew ₦500", i),
                Err(e) => println!("Thread {}: ✗ {}", i, e),
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_balance = bank.get_balance(&account_num).unwrap();
    println!("\nFinal balance: ₦{:.2}", final_balance as f64 / 100.0);
    println!("Expected: ₦5,000 (10,000 - 5x500, as 5 should fail)");
    
    // The mutex ensures that even with concurrent access,
    // we never overdraw the account or lose track of money!
}
```

## Real-World Scenarios

### Scenario 1: Salary Processing
```rust
fn process_monthly_salaries(bank: &BankingSystem, company_account: &str) {
    // Company salaries to pay
    let salaries = vec![
        ("EMP001_SAVINGS", 500_000),  // ₦5,000
        ("EMP002_CURRENT", 750_000),  // ₦7,500
        ("EMP003_SAVINGS", 600_000),  // ₦6,000
    ];
    
    println!("Processing {} salaries from company account...", salaries.len());
    
    for (employee_account, amount) in salaries {
        match bank.transfer(
            company_account,
            employee_account,
            amount,
            "Monthly Salary".to_string()
        ) {
            Ok(txn) => println!("✓ Paid ₦{:.2} to {}", 
                               amount as f64 / 100.0, employee_account),
            Err(e) => println!("✗ Failed to pay {}: {}", employee_account, e),
        }
    }
    
    println!("Salary processing complete!");
}
```

### Scenario 2: Bill Payment
```rust
fn pay_utility_bill(bank: &BankingSystem, 
                   customer_account: &str,
                   utility_company: &str,
                   bill_amount: i64,
                   bill_reference: &str) -> BankingResult<Transaction> {
    
    println!("Processing bill payment...");
    println!("Customer: {}", customer_account);
    println!("Biller: {}", utility_company);
    println!("Amount: ₦{:.2}", bill_amount as f64 / 100.0);
    println!("Reference: {}", bill_reference);
    
    // Transfer from customer to utility company
    let transaction = bank.transfer(
        customer_account,
        utility_company,
        bill_amount,
        format!("Bill Payment - Ref: {}", bill_reference)
    )?;
    
    println!("✓ Bill paid successfully!");
    println!("Transaction ID: {}", transaction.transaction_id);
    
    Ok(transaction)
}
```

### Scenario 3: ATM Withdrawal with Daily Limit
```rust
fn atm_withdrawal_with_limit(
    bank: &BankingSystem,
    account_number: &str,
    amount: i64,
    daily_limit: i64
) -> BankingResult<Transaction> {
    
    // Get today's withdrawals
    let history = bank.get_transaction_history(account_number);
    let today = Utc::now().date_naive();
    
    let today_withdrawals: i64 = history.iter()
        .filter(|t| {
            t.transaction_type == TransactionType::Withdrawal &&
            t.created_at.date_naive() == today &&
            t.status == TransactionStatus::Completed
        })
        .map(|t| t.amount)
        .sum();
    
    println!("ATM Withdrawal Request:");
    println!("Amount: ₦{:.2}", amount as f64 / 100.0);
    println!("Today's withdrawals: ₦{:.2}", today_withdrawals as f64 / 100.0);
    println!("Daily limit: ₦{:.2}", daily_limit as f64 / 100.0);
    
    if today_withdrawals + amount > daily_limit {
        return Err(BankingError::TransactionFailed(
            format!("Daily withdrawal limit exceeded. Remaining: ₦{:.2}",
                   (daily_limit - today_withdrawals) as f64 / 100.0)
        ));
    }
    
    // Process withdrawal
    bank.withdraw(account_number, amount, "ATM Withdrawal".to_string())
}
```

## Debugging Exercises

### Exercise 9: Find the Bug
```rust
// This code has a bug. Can you find it?
fn buggy_transfer(bank: &BankingSystem, from: &str, to: &str, amount: i64) {
    let from_balance = bank.get_balance(from).unwrap();
    
    if from_balance > amount {  // BUG: What about minimum balance?
        bank.transfer(from, to, amount, "Transfer".to_string()).unwrap();
    } else {
        println!("Insufficient funds!");
    }
}

// Fix: Should check: from_balance - amount >= minimum_balance
```

### Exercise 10: Optimize This Code
```rust
// This code works but is inefficient. How can you improve it?
fn get_large_transactions(bank: &BankingSystem, threshold: i64) -> Vec<Transaction> {
    let mut large_txns = Vec::new();
    
    for txn in bank.get_all_transactions() {
        if txn.amount >= threshold {
            large_txns.push(txn);
        }
    }
    
    large_txns
}

// Better: Use iterator methods
fn get_large_transactions_optimized(bank: &BankingSystem, threshold: i64) -> Vec<Transaction> {
    bank.get_all_transactions()
        .into_iter()
        .filter(|t| t.amount >= threshold)
        .collect()
}
```

## Challenge Projects

### Challenge 1: Add Loan Management
**Requirements:**
1. Create a Loan struct with principal, interest rate, term
2. Implement disbursement (credit customer account)
3. Implement repayment schedule calculation
4. Process monthly repayments

### Challenge 2: Implement Standing Orders
**Requirements:**
1. Create StandingOrder struct
2. Store recurring payment instructions
3. Implement daily batch to execute due orders
4. Handle failures gracefully

### Challenge 3: Add Multi-Currency Support
**Requirements:**
1. Support USD, GBP, EUR accounts
2. Implement exchange rates
3. Add currency conversion for transfers
4. Handle exchange rate fluctuations

### Challenge 4: Fraud Detection System
**Requirements:**
1. Monitor unusual transaction patterns
2. Flag large withdrawals
3. Detect rapid multiple transactions
4. Alert on suspicious activity

### Challenge 5: Database Integration
**Requirements:**
1. Replace in-memory storage with PostgreSQL
2. Implement transaction (ACID properties)
3. Add database migration system
4. Implement connection pooling

## Testing Exercises

### Unit Test Example
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deposit_increases_balance() {
        let bank = BankingSystem::new(100_000);
        let account = bank.create_account(
            "TEST".to_string(),
            AccountType::Savings,
            500_000,
            "NGN".to_string()
        ).unwrap();
        
        let initial = bank.get_balance(&account.account_number).unwrap();
        
        bank.deposit(&account.account_number, 100_000, "Test".to_string()).unwrap();
        
        let final_balance = bank.get_balance(&account.account_number).unwrap();
        
        assert_eq!(final_balance, initial + 100_000);
    }
    
    #[test]
    fn test_withdrawal_insufficient_funds() {
        let bank = BankingSystem::new(100_000);
        let account = bank.create_account(
            "TEST".to_string(),
            AccountType::Current,
            200_000,
            "NGN".to_string()
        ).unwrap();
        
        // Try to withdraw more than available (considering minimum balance)
        let result = bank.withdraw(&account.account_number, 150_000, "Test".to_string());
        
        assert!(result.is_err());
        assert!(matches!(result, Err(BankingError::InsufficientFunds(_))));
    }
    
    #[test]
    fn test_transfer_maintains_system_balance() {
        let bank = BankingSystem::new(100_000);
        
        let acc1 = bank.create_account("A".to_string(), AccountType::Current,
                                        1_000_000, "NGN".to_string()).unwrap();
        let acc2 = bank.create_account("B".to_string(), AccountType::Savings,
                                        500_000, "NGN".to_string()).unwrap();
        
        let total_before = bank.get_balance(&acc1.account_number).unwrap() +
                          bank.get_balance(&acc2.account_number).unwrap();
        
        bank.transfer(&acc1.account_number, &acc2.account_number,
                     300_000, "Test".to_string()).unwrap();
        
        let total_after = bank.get_balance(&acc1.account_number).unwrap() +
                         bank.get_balance(&acc2.account_number).unwrap();
        
        assert_eq!(total_before, total_after);
    }
}
```

## Learning Path

**Week 1: Basics**
- Understand account creation
- Practice deposits and withdrawals
- Learn error handling

**Week 2: Transfers**
- Understand double-entry bookkeeping
- Practice transfers
- Analyze transaction history

**Week 3: Advanced Features**
- Interest calculation
- Fees and charges
- Transaction limits

**Week 4: Concurrency**
- Thread safety
- Race conditions
- Mutex usage

**Week 5: Production Features**
- Database integration
- API development
- Security implementation

**Week 6: Final Project**
- Build a complete feature
- Write comprehensive tests
- Deploy and document

## Additional Resources

1. **Rust Book**: https://doc.rust-lang.org/book/
2. **Rust By Example**: https://doc.rust-lang.org/rust-by-example/
3. **Banking Basics**: Study double-entry bookkeeping
4. **CBN Guidelines**: https://www.cbn.gov.ng/
5. **Finacle Documentation**: Research core banking systems

Happy Learning! 🚀
