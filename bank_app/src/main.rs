use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ============================================================================
// CORE DATA STRUCTURES
// ============================================================================

/// Represents different types of accounts in the banking system
#[derive(Debug, Clone, PartialEq)]
pub enum AccountType {
    Savings,
    Current,
    Fixed,
    Loan,
}

/// Represents the status of an account
#[derive(Debug, Clone, PartialEq)]
pub enum AccountStatus {
    Active,
    Dormant,
    Frozen,
    Closed,
}

/// Core account structure - represents a customer's bank account
/// Uses Rust's type system to ensure data integrity
#[derive(Debug, Clone)]
pub struct Account {
    pub account_number: String,
    pub customer_id: String,
    pub account_type: AccountType,
    pub balance: i64, // Stored in minor units (kobo for Naira) to avoid floating point errors
    pub currency: String,
    pub status: AccountStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Account {
    /// Creates a new account with initial balance
    pub fn new(
        customer_id: String,
        account_type: AccountType,
        initial_balance: i64,
        currency: String,
    ) -> Self {
        let now = Utc::now();
        Account {
            account_number: format!("ACC{}", Uuid::new_v4().to_string().replace("-", "")[..10].to_uppercase()),
            customer_id,
            account_type,
            balance: initial_balance,
            currency,
            status: AccountStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }

    /// Converts minor units to major units for display (kobo to Naira)
    pub fn balance_as_decimal(&self) -> f64 {
        self.balance as f64 / 100.0
    }
}

// ============================================================================
// TRANSACTION SYSTEM
// ============================================================================

/// Transaction types in the banking system
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
    InterestCredit,
    Fee,
    Reversal,
}

/// Transaction status for tracking and reconciliation
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
}

/// Core transaction record - implements double-entry bookkeeping
/// Every transaction affects at least one account (debit/credit)
#[derive(Debug, Clone)]
pub struct Transaction {
    pub transaction_id: String,
    pub transaction_type: TransactionType,
    pub from_account: Option<String>,
    pub to_account: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub description: String,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub reference: String,
}

impl Transaction {
    pub fn new(
        transaction_type: TransactionType,
        from_account: Option<String>,
        to_account: Option<String>,
        amount: i64,
        currency: String,
        description: String,
    ) -> Self {
        Transaction {
            transaction_id: format!("TXN{}", Uuid::new_v4().to_string().replace("-", "")[..12].to_uppercase()),
            transaction_type,
            from_account,
            to_account,
            amount,
            currency,
            description,
            status: TransactionStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
            reference: format!("REF{}", Uuid::new_v4().to_string()[..8].to_uppercase()),
        }
    }
}

// ============================================================================
// BANKING OPERATIONS & BUSINESS LOGIC
// ============================================================================

/// Result type for banking operations
pub type BankingResult<T> = Result<T, BankingError>;

/// Custom error types for banking operations
#[derive(Debug, Clone)]
pub enum BankingError {
    AccountNotFound(String),
    InsufficientFunds(String),
    AccountInactive(String),
    InvalidAmount(String),
    TransactionFailed(String),
    CurrencyMismatch(String),
}

impl std::fmt::Display for BankingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BankingError::AccountNotFound(msg) => write!(f, "Account not found: {}", msg),
            BankingError::InsufficientFunds(msg) => write!(f, "Insufficient funds: {}", msg),
            BankingError::AccountInactive(msg) => write!(f, "Account inactive: {}", msg),
            BankingError::InvalidAmount(msg) => write!(f, "Invalid amount: {}", msg),
            BankingError::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
            BankingError::CurrencyMismatch(msg) => write!(f, "Currency mismatch: {}", msg),
        }
    }
}

impl std::error::Error for BankingError {}

// ============================================================================
// CORE BANKING ENGINE
// ============================================================================

/// The main banking system - manages all accounts and transactions
/// Uses Arc<Mutex<>> for thread-safe concurrent access (critical for banking systems)
pub struct BankingSystem {
    accounts: Arc<Mutex<HashMap<String, Account>>>,
    transactions: Arc<Mutex<Vec<Transaction>>>,
    minimum_balance: i64, // Minimum balance requirement in minor units
}

impl BankingSystem {
    /// Creates a new banking system instance
    pub fn new(minimum_balance: i64) -> Self {
        BankingSystem {
            accounts: Arc::new(Mutex::new(HashMap::new())),
            transactions: Arc::new(Mutex::new(Vec::new())),
            minimum_balance,
        }
    }

    // ========================================================================
    // ACCOUNT MANAGEMENT OPERATIONS
    // ========================================================================

    /// Creates a new account in the system
    pub fn create_account(
        &self,
        customer_id: String,
        account_type: AccountType,
        initial_balance: i64,
        currency: String,
    ) -> BankingResult<Account> {
        if initial_balance < 0 {
            return Err(BankingError::InvalidAmount("Initial balance cannot be negative".to_string()));
        }

        let account = Account::new(customer_id, account_type, initial_balance, currency);
        
        let mut accounts = self.accounts.lock().unwrap();
        let account_number = account.account_number.clone();
        accounts.insert(account_number.clone(), account.clone());

        println!("✓ Account created: {} with balance: ₦{:.2}", 
                 account_number, account.balance_as_decimal());
        
        Ok(account)
    }

    /// Retrieves account information
    pub fn get_account(&self, account_number: &str) -> BankingResult<Account> {
        let accounts = self.accounts.lock().unwrap();
        accounts.get(account_number)
            .cloned()
            .ok_or_else(|| BankingError::AccountNotFound(account_number.to_string()))
    }

    /// Checks if an account is active and can perform transactions
    fn validate_account(&self, account_number: &str) -> BankingResult<()> {
        let account = self.get_account(account_number)?;
        
        if account.status != AccountStatus::Active {
            return Err(BankingError::AccountInactive(
                format!("Account {} is {:?}", account_number, account.status)
            ));
        }
        
        Ok(())
    }

    // ========================================================================
    // TRANSACTION OPERATIONS - Core Banking Functions
    // ========================================================================

    /// Deposits money into an account
    /// This is a single-leg transaction (only credits one account)
    pub fn deposit(
        &self,
        account_number: &str,
        amount: i64,
        description: String,
    ) -> BankingResult<Transaction> {
        // Validation
        if amount <= 0 {
            return Err(BankingError::InvalidAmount("Deposit amount must be positive".to_string()));
        }

        self.validate_account(account_number)?;

        // Create transaction record
        let mut transaction = Transaction::new(
            TransactionType::Deposit,
            None,
            Some(account_number.to_string()),
            amount,
            "NGN".to_string(),
            description,
        );

        // Execute the deposit (credit the account)
        {
            let mut accounts = self.accounts.lock().unwrap();
            if let Some(account) = accounts.get_mut(account_number) {
                account.balance += amount;
                account.updated_at = Utc::now();
                
                // Mark transaction as completed
                transaction.status = TransactionStatus::Completed;
                transaction.completed_at = Some(Utc::now());

                println!("✓ Deposit: ₦{:.2} to {} | New Balance: ₦{:.2}", 
                         amount as f64 / 100.0, account_number, account.balance_as_decimal());
            }
        }

        // Record transaction
        self.transactions.lock().unwrap().push(transaction.clone());

        Ok(transaction)
    }

    /// Withdraws money from an account
    /// Implements balance checks and minimum balance requirements
    pub fn withdraw(
        &self,
        account_number: &str,
        amount: i64,
        description: String,
    ) -> BankingResult<Transaction> {
        // Validation
        if amount <= 0 {
            return Err(BankingError::InvalidAmount("Withdrawal amount must be positive".to_string()));
        }

        self.validate_account(account_number)?;

        // Check sufficient balance
        let account = self.get_account(account_number)?;
        let new_balance = account.balance - amount;
        
        if new_balance < self.minimum_balance {
            return Err(BankingError::InsufficientFunds(
                format!("Insufficient funds. Available: ₦{:.2}, Required: ₦{:.2} (including minimum balance)", 
                        (account.balance - self.minimum_balance) as f64 / 100.0,
                        amount as f64 / 100.0)
            ));
        }

        // Create transaction record
        let mut transaction = Transaction::new(
            TransactionType::Withdrawal,
            Some(account_number.to_string()),
            None,
            amount,
            "NGN".to_string(),
            description,
        );

        // Execute the withdrawal (debit the account)
        {
            let mut accounts = self.accounts.lock().unwrap();
            if let Some(account) = accounts.get_mut(account_number) {
                account.balance -= amount;
                account.updated_at = Utc::now();
                
                transaction.status = TransactionStatus::Completed;
                transaction.completed_at = Some(Utc::now());

                println!("✓ Withdrawal: ₦{:.2} from {} | New Balance: ₦{:.2}", 
                         amount as f64 / 100.0, account_number, account.balance_as_decimal());
            }
        }

        // Record transaction
        self.transactions.lock().unwrap().push(transaction.clone());

        Ok(transaction)
    }

    /// Transfers money between two accounts
    /// This implements double-entry bookkeeping - debit one account, credit another
    /// This is atomic - either both succeed or both fail
    pub fn transfer(
        &self,
        from_account: &str,
        to_account: &str,
        amount: i64,
        description: String,
    ) -> BankingResult<Transaction> {
        // Validation
        if amount <= 0 {
            return Err(BankingError::InvalidAmount("Transfer amount must be positive".to_string()));
        }

        if from_account == to_account {
            return Err(BankingError::TransactionFailed("Cannot transfer to same account".to_string()));
        }

        self.validate_account(from_account)?;
        self.validate_account(to_account)?;

        // Check sufficient balance
        let source_account = self.get_account(from_account)?;
        let new_balance = source_account.balance - amount;
        
        if new_balance < self.minimum_balance {
            return Err(BankingError::InsufficientFunds(
                format!("Insufficient funds in account {}", from_account)
            ));
        }

        // Check currency match
        let dest_account = self.get_account(to_account)?;
        if source_account.currency != dest_account.currency {
            return Err(BankingError::CurrencyMismatch(
                "Accounts have different currencies".to_string()
            ));
        }

        // Create transaction record
        let mut transaction = Transaction::new(
            TransactionType::Transfer,
            Some(from_account.to_string()),
            Some(to_account.to_string()),
            amount,
            source_account.currency.clone(),
            description,
        );

        // Execute the transfer atomically
        // In a real system, this would use database transactions
        {
            let mut accounts = self.accounts.lock().unwrap();
            
            // Debit source account
            if let Some(from_acc) = accounts.get_mut(from_account) {
                from_acc.balance -= amount;
                from_acc.updated_at = Utc::now();
            }
            
            // Credit destination account
            if let Some(to_acc) = accounts.get_mut(to_account) {
                to_acc.balance += amount;
                to_acc.updated_at = Utc::now();
            }

            transaction.status = TransactionStatus::Completed;
            transaction.completed_at = Some(Utc::now());

            println!("✓ Transfer: ₦{:.2} from {} to {}", 
                     amount as f64 / 100.0, from_account, to_account);
        }

        // Record transaction
        self.transactions.lock().unwrap().push(transaction.clone());

        Ok(transaction)
    }

    // ========================================================================
    // QUERY & REPORTING OPERATIONS
    // ========================================================================

    /// Gets account balance
    pub fn get_balance(&self, account_number: &str) -> BankingResult<i64> {
        let account = self.get_account(account_number)?;
        Ok(account.balance)
    }

    /// Retrieves transaction history for an account
    pub fn get_transaction_history(&self, account_number: &str) -> Vec<Transaction> {
        let transactions = self.transactions.lock().unwrap();
        transactions.iter()
            .filter(|t| {
                t.from_account.as_ref() == Some(&account_number.to_string()) ||
                t.to_account.as_ref() == Some(&account_number.to_string())
            })
            .cloned()
            .collect()
    }

    /// Gets all transactions in the system (for audit purposes)
    pub fn get_all_transactions(&self) -> Vec<Transaction> {
        self.transactions.lock().unwrap().clone()
    }

    /// Generates a mini statement for an account
    pub fn generate_mini_statement(&self, account_number: &str, limit: usize) -> BankingResult<()> {
        let account = self.get_account(account_number)?;
        let mut history = self.get_transaction_history(account_number);
        
        // Sort by date, most recent first
        history.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        history.truncate(limit);

        println!("\n╔════════════════════════════════════════════════════════════════╗");
        println!("║               MINI STATEMENT - LAST {} TRANSACTIONS              ║", limit);
        println!("╠════════════════════════════════════════════════════════════════╣");
        println!("║ Account: {}                                         ║", account_number);
        println!("║ Current Balance: ₦{:>10.2}                              ║", account.balance_as_decimal());
        println!("╠════════════════════════════════════════════════════════════════╣");

        for txn in history {
            let txn_type = format!("{:?}", txn.transaction_type);
            let amount_display = format!("₦{:.2}", txn.amount as f64 / 100.0);
            let date = txn.created_at.format("%Y-%m-%d %H:%M");
            
            println!("║ {} | {:>12} | {} ║", 
                     date, amount_display, txn_type);
            println!("║   {} ║", txn.description);
        }

        println!("╚════════════════════════════════════════════════════════════════╝\n");

        Ok(())
    }
}

// ============================================================================
// DEMONSTRATION & TESTING
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║     EDUCATIONAL BANKING SYSTEM - RUST IMPLEMENTATION             ║");
    println!("║     Demonstrating Core Banking Concepts                          ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Initialize banking system with minimum balance of ₦1,000 (100,000 kobo)
    let bank = BankingSystem::new(100_000);

    println!("═══ ACCOUNT CREATION ═══\n");
    
    // Create accounts for customers
    let account1 = bank.create_account(
        "CUST001".to_string(),
        AccountType::Savings,
        500_000, // ₦5,000 in kobo
        "NGN".to_string(),
    ).unwrap();

    let account2 = bank.create_account(
        "CUST002".to_string(),
        AccountType::Current,
        1_000_000, // ₦10,000 in kobo
        "NGN".to_string(),
    ).unwrap();

    let account3 = bank.create_account(
        "CUST003".to_string(),
        AccountType::Savings,
        200_000, // ₦2,000 in kobo
        "NGN".to_string(),
    ).unwrap();

    println!("\n═══ DEPOSIT OPERATIONS ═══\n");
    
    // Perform deposits
    bank.deposit(
        &account1.account_number,
        250_000, // ₦2,500
        "Cash deposit at branch".to_string(),
    ).unwrap();

    bank.deposit(
        &account2.account_number,
        500_000, // ₦5,000
        "Salary credit".to_string(),
    ).unwrap();

    println!("\n═══ WITHDRAWAL OPERATIONS ═══\n");
    
    // Perform withdrawals
    bank.withdraw(
        &account1.account_number,
        100_000, // ₦1,000
        "ATM withdrawal".to_string(),
    ).unwrap();

    // Attempt withdrawal with insufficient funds
    println!("\nAttempting withdrawal with insufficient funds...");
    match bank.withdraw(&account3.account_number, 150_000, "Large withdrawal".to_string()) {
        Ok(_) => println!("Withdrawal successful"),
        Err(e) => println!("✗ {}", e),
    }

    println!("\n═══ TRANSFER OPERATIONS ═══\n");
    
    // Perform transfers
    bank.transfer(
        &account2.account_number,
        &account1.account_number,
        300_000, // ₦3,000
        "Payment for services".to_string(),
    ).unwrap();

    bank.transfer(
        &account1.account_number,
        &account3.account_number,
        200_000, // ₦2,000
        "Gift transfer".to_string(),
    ).unwrap();

    println!("\n═══ ACCOUNT BALANCES ═══\n");
    
    // Check final balances
    let balance1 = bank.get_balance(&account1.account_number).unwrap();
    let balance2 = bank.get_balance(&account2.account_number).unwrap();
    let balance3 = bank.get_balance(&account3.account_number).unwrap();

    println!("Account {} balance: ₦{:.2}", account1.account_number, balance1 as f64 / 100.0);
    println!("Account {} balance: ₦{:.2}", account2.account_number, balance2 as f64 / 100.0);
    println!("Account {} balance: ₦{:.2}", account3.account_number, balance3 as f64 / 100.0);

    println!("\n═══ TRANSACTION HISTORY ═══\n");
    
    // Generate mini statement
    bank.generate_mini_statement(&account1.account_number, 5).unwrap();

    println!("═══ AUDIT TRAIL - ALL TRANSACTIONS ═══\n");
    
    let all_transactions = bank.get_all_transactions();
    println!("Total transactions in system: {}", all_transactions.len());
    println!("\nTransaction IDs:");
    for txn in all_transactions {
        println!("  {} - {:?} - Status: {:?}", 
                 txn.transaction_id, txn.transaction_type, txn.status);
    }

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    DEMONSTRATION COMPLETED                       ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
}
