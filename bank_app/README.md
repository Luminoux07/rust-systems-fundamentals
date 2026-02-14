# Educational Banking System in Rust

## Overview
This is an educational implementation of core banking concepts similar to systems like Finacle used by Nigerian banks. It demonstrates transaction processing, account management, and core banking operations using Rust's safety and concurrency features.

## Key Banking Concepts Implemented

### 1. **Account Management**
- **Account Types**: Different account categories (Savings, Current, Fixed, Loan)
- **Account Status**: Lifecycle management (Active, Dormant, Frozen, Closed)
- **Account Numbers**: Unique identifiers for each account
- **Customer Linking**: Each account is linked to a customer ID

### 2. **Money Representation**
```rust
pub balance: i64, // Stored in minor units (kobo)
```
**Why?** Floating-point arithmetic can introduce rounding errors in financial calculations.
- ₦1,000.00 is stored as 100,000 kobo (minor units)
- All calculations use integers for precision
- Convert to decimal only for display purposes

### 3. **Transaction Types**

#### **Deposit** (Single-Leg Transaction)
- Credits money INTO an account
- No debit side (cash coming from outside the bank)
- Increases account balance

#### **Withdrawal** (Single-Leg Transaction)
- Debits money FROM an account
- No credit side (cash leaving the bank)
- Decreases account balance
- Requires balance validation

#### **Transfer** (Double-Leg Transaction)
- **Double-Entry Bookkeeping**: Every debit must have a corresponding credit
- Debits source account, Credits destination account
- Atomic operation - both must succeed or both fail
- Maintains system balance (sum of all accounts unchanged)

### 4. **Double-Entry Bookkeeping**
```
Transfer ₦1,000 from Account A to Account B:
  Debit Account A: -₦1,000
  Credit Account B: +₦1,000
  Net Change: ₦0
```
This ensures the bank's books always balance.

### 5. **Transaction Lifecycle**
```
Pending → Completed / Failed / Reversed
```
- **Pending**: Transaction initiated but not processed
- **Completed**: Successfully processed
- **Failed**: Validation failed or processing error
- **Reversed**: Transaction undone (for corrections)

### 6. **Business Rules & Validations**

#### **Minimum Balance Requirement**
```rust
if new_balance < self.minimum_balance {
    return Err(InsufficientFunds);
}
```
Nigerian banks often require minimum balances to keep accounts active.

#### **Account Status Checks**
```rust
if account.status != AccountStatus::Active {
    return Err(AccountInactive);
}
```
Only active accounts can perform transactions.

#### **Currency Matching**
```rust
if source_account.currency != dest_account.currency {
    return Err(CurrencyMismatch);
}
```
Prevents accidental transfers between different currency accounts.

### 7. **Concurrency & Thread Safety**

```rust
Arc<Mutex<HashMap<String, Account>>>
```

**Arc (Atomic Reference Counting)**:
- Allows multiple threads to share ownership
- Essential for banking systems handling concurrent transactions

**Mutex (Mutual Exclusion)**:
- Ensures only one thread modifies data at a time
- Prevents race conditions in balance updates
- Critical for preventing double-spending

**Real-world scenario**:
```
Thread 1: Withdraw ₦1,000 from Account A
Thread 2: Withdraw ₦500 from Account A
```
Without Mutex, both might read balance as ₦2,000 and both succeed, overdrawing the account.
With Mutex, operations are serialized and the second one validates against the updated balance.

### 8. **Audit Trail**
Every transaction is recorded with:
- Unique transaction ID
- Timestamp (created and completed)
- Reference number
- All parties involved
- Amount and currency
- Status history

This is required for:
- Regulatory compliance
- Dispute resolution
- Fraud detection
- Reconciliation

### 9. **Error Handling**

```rust
pub type BankingResult<T> = Result<T, BankingError>;
```

Rust's Result type forces explicit error handling:
- Can't ignore errors (compile-time safety)
- Clear error types for different failure modes
- Better than exceptions in critical systems

### 10. **Statement Generation**
Mini statements provide:
- Recent transaction history
- Current balance
- Transaction details
- Audit information

## Rust-Specific Advantages for Banking

### 1. **Memory Safety**
- No null pointer dereferencing
- No buffer overflows
- No use-after-free bugs
- Zero-cost abstractions

### 2. **Type Safety**
```rust
enum AccountType {
    Savings,
    Current,
}
```
Impossible to have invalid account types at runtime.

### 3. **Ownership System**
```rust
let account = bank.create_account(...);
// account is moved, can't be used again without explicit cloning
```
Prevents accidental data corruption.

### 4. **Pattern Matching**
```rust
match transaction.status {
    TransactionStatus::Completed => process(),
    TransactionStatus::Failed => rollback(),
    _ => log_error(),
}
```
Compiler ensures all cases are handled.

## How Real Banking Systems Differ

### This Educational Version
- In-memory storage (data lost on restart)
- Simple locking mechanism
- Basic validation
- No persistence layer

### Production Systems (like Finacle)
- **Database Persistence**: PostgreSQL, Oracle, etc.
- **ACID Transactions**: Database-level atomicity
- **Distributed Systems**: Multiple servers, load balancing
- **High Availability**: Failover, replication
- **Security**: Encryption, authentication, authorization
- **Regulatory Compliance**: CBN requirements, anti-money laundering
- **Integration**: SWIFT, NIBSS, card networks
- **Complex Products**: Loans, investments, standing orders
- **Batch Processing**: End-of-day reconciliation
- **Real-time Processing**: Instant payments
- **Reporting**: Regulatory reports, management dashboards

## Running the System

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone or create the project
cd banking_system
```

### Build and Run
```bash
# Build the project
cargo build --release

# Run the demonstration
cargo run

# Run tests (if added)
cargo test
```

## Architecture Patterns

### 1. **Repository Pattern**
`BankingSystem` acts as a repository for accounts and transactions.

### 2. **Command Pattern**
Each operation (deposit, withdraw, transfer) is a command that can be:
- Validated
- Executed
- Logged
- Reversed (if needed)

### 3. **Event Sourcing (Implicit)**
All transactions are stored, creating an event log of all changes.

## Key Banking Workflows

### Opening an Account
```
Customer → Bank → Validate ID → Create Account → Issue Account Number
```

### Processing a Transfer
```
1. Validate source account exists and is active
2. Validate destination account exists and is active
3. Check sufficient balance
4. Check currency match
5. Lock accounts (Mutex)
6. Debit source
7. Credit destination
8. Record transaction
9. Unlock accounts
10. Return confirmation
```

### End-of-Day Reconciliation (Not Implemented)
```
1. Sum all debits
2. Sum all credits
3. Verify: Total Debits = Total Credits
4. Generate reports
5. Archive transactions
```

## Security Considerations (Not Fully Implemented)

Production systems need:
- **Authentication**: Who is making the request?
- **Authorization**: Are they allowed to do this?
- **Encryption**: Data at rest and in transit
- **Rate Limiting**: Prevent abuse
- **Fraud Detection**: Unusual patterns
- **Audit Logging**: Who did what, when?

## Testing Strategies

### Unit Tests
Test individual functions:
```rust
#[test]
fn test_deposit_increases_balance() {
    // Test implementation
}
```

### Integration Tests
Test multiple operations together:
```rust
#[test]
fn test_transfer_workflow() {
    // Create accounts, transfer, verify balances
}
```

### Concurrent Tests
Test thread safety:
```rust
#[test]
fn test_concurrent_withdrawals() {
    // Multiple threads withdrawing from same account
}
```

## Extending This System

### Suggested Enhancements
1. **Database Integration**: Use PostgreSQL with Diesel or SQLx
2. **Web API**: Add REST API with Actix or Axum
3. **Interest Calculation**: Daily, monthly compounding
4. **Loan Management**: Disbursement, repayment schedules
5. **Standing Orders**: Recurring payments
6. **Cards**: Debit/credit card operations
7. **Mobile Integration**: USSD, mobile app APIs
8. **Inter-bank Transfers**: NIBSS integration
9. **Bill Payments**: Electricity, airtime, etc.
10. **KYC Management**: Customer verification

## Compliance Requirements (Nigerian Context)

### Central Bank of Nigeria (CBN) Requirements
- **Know Your Customer (KYC)**: Customer identification
- **Anti-Money Laundering (AML)**: Transaction monitoring
- **Transaction Limits**: Daily/monthly limits
- **Reporting**: Suspicious transactions, large cash transactions
- **Data Retention**: Transaction records for specified periods
- **Business Continuity**: Disaster recovery plans

## Performance Considerations

### This Implementation
- Single-process
- In-memory (fast but volatile)
- Simple locking (can be a bottleneck)

### Production Optimizations
- **Sharding**: Distribute accounts across databases
- **Caching**: Redis for frequently accessed data
- **Read Replicas**: Separate read and write databases
- **Message Queues**: Async processing for non-critical operations
- **Connection Pooling**: Reuse database connections

## Common Banking Terminology

- **Debit**: Decrease in liability or increase in asset (money going out)
- **Credit**: Increase in liability or decrease in asset (money coming in)
- **Clearing**: Processing and settlement of transactions
- **Reconciliation**: Matching records across systems
- **Nostro Account**: Bank's account held in another bank
- **Vostro Account**: Another bank's account held in your bank
- **Core Banking**: Central system managing accounts and transactions

## Learning Resources

1. **Rust Programming**: https://doc.rust-lang.org/book/
2. **Concurrency in Rust**: https://doc.rust-lang.org/book/ch16-00-concurrency.html
3. **Banking Systems**: "The Principles of Banking" by Moorad Choudhry
4. **Double-Entry Bookkeeping**: Basic accounting principles
5. **CBN Regulations**: https://www.cbn.gov.ng/

## Conclusion

This educational system demonstrates core banking concepts using Rust's powerful type system and safety guarantees. While simplified compared to production systems like Finacle, it covers the fundamental principles that power modern banking operations.

The key takeaways are:
- Financial precision requires careful data representation
- Concurrency control is critical in multi-user systems
- Validation and error handling prevent data corruption
- Audit trails enable accountability and compliance
- Type safety and memory safety reduce bugs in critical systems
