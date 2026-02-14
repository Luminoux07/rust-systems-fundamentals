# Banking System Architecture & Concepts

## System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        CLIENT LAYER                             │
│  (ATM, Mobile App, Web Banking, Branch Teller, API)             │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    APPLICATION LAYER                            │
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Account    │  │ Transaction  │  │   Reporting  │         │
│  │  Management  │  │  Processing  │  │   & Audit    │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Interest   │  │    Fees &    │  │   Standing   │         │
│  │ Calculation  │  │   Charges    │  │    Orders    │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    BUSINESS LOGIC LAYER                         │
│                      (BankingSystem)                            │
│                                                                 │
│  • Validation Rules                                             │
│  • Transaction Processing                                       │
│  • Balance Management                                           │
│  • Concurrency Control (Arc<Mutex<>>)                          │
│  • Error Handling                                               │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                       DATA LAYER                                │
│                                                                 │
│  ┌──────────────────────┐      ┌──────────────────────┐        │
│  │   Account Storage    │      │  Transaction Storage │        │
│  │  HashMap<String,     │      │  Vec<Transaction>    │        │
│  │     Account>         │      │                      │        │
│  └──────────────────────┘      └──────────────────────┘        │
│                                                                 │
│  In Production: Replace with Database (PostgreSQL, Oracle)     │
└─────────────────────────────────────────────────────────────────┘
```

## Transaction Processing Flow

### 1. Deposit Transaction Flow

```
┌──────────┐
│  Client  │
│ (ATM/App)│
└────┬─────┘
     │
     │ 1. Deposit Request (Account#, Amount)
     ▼
┌────────────────┐
│  Validation    │
│  • Amount > 0  │
│  • Account     │
│    Active      │
└────┬───────────┘
     │ ✓ Valid
     ▼
┌────────────────┐
│ Create         │
│ Transaction    │
│ Record         │
└────┬───────────┘
     │
     ▼
┌────────────────┐
│ Lock Account   │
│ (Mutex.lock()) │
└────┬───────────┘
     │
     ▼
┌────────────────┐
│ Credit Account │
│ balance += amt │
└────┬───────────┘
     │
     ▼
┌────────────────┐
│ Update         │
│ Timestamp      │
└────┬───────────┘
     │
     ▼
┌────────────────┐
│ Mark Txn       │
│ Complete       │
└────┬───────────┘
     │
     ▼
┌────────────────┐
│ Unlock Account │
│(Mutex.unlock())│
└────┬───────────┘
     │
     ▼
┌────────────────┐
│ Log Transaction│
└────┬───────────┘
     │
     ▼ Success
┌────────────────┐
│ Return Receipt │
└────────────────┘
```

### 2. Transfer Transaction Flow (Double-Entry)

```
Transfer: Account A → Account B

┌──────────────────────────────────────────────────────────┐
│                    VALIDATION PHASE                      │
└──────────────────────────────────────────────────────────┘
    │
    ├─► Validate Amount > 0
    ├─► Validate A ≠ B
    ├─► Validate Account A Active
    ├─► Validate Account B Active
    ├─► Validate A has Sufficient Balance
    └─► Validate Currency Match
         │
         ▼ All Valid
┌──────────────────────────────────────────────────────────┐
│                   EXECUTION PHASE                        │
└──────────────────────────────────────────────────────────┘
    │
    ├─► Lock Both Accounts (Mutex)
    │
    ├─► Debit Account A (balance -= amount)
    │     └─► DEBIT SIDE of Double Entry
    │
    ├─► Credit Account B (balance += amount)
    │     └─► CREDIT SIDE of Double Entry
    │
    ├─► Update Timestamps
    │
    └─► Unlock Both Accounts
         │
         ▼
┌──────────────────────────────────────────────────────────┐
│                   RECORDING PHASE                        │
└──────────────────────────────────────────────────────────┘
    │
    ├─► Create Transaction Record
    │     • Transaction ID
    │     • From: Account A
    │     • To: Account B
    │     • Amount
    │     • Status: Completed
    │     • Timestamp
    │
    └─► Add to Transaction Log
         │
         ▼ Success
    Return Transaction Receipt
```

## Double-Entry Bookkeeping Explained

### Basic Principle
Every financial transaction affects at least TWO accounts with EQUAL amounts.
The sum of all debits MUST equal the sum of all credits.

### Example: Transfer ₦5,000 from Alice to Bob

```
BEFORE:
┌──────────────┬──────────┐    ┌──────────────┬──────────┐
│ Alice's Acc  │ ₦10,000  │    │  Bob's Acc   │ ₦3,000   │
└──────────────┴──────────┘    └──────────────┴──────────┘

TRANSACTION:
┌──────────────┬──────────┬──────────┐
│   Account    │  Debit   │  Credit  │
├──────────────┼──────────┼──────────┤
│ Alice        │ ₦5,000   │    -     │  ← Money leaving
│ Bob          │    -     │ ₦5,000   │  ← Money arriving
└──────────────┴──────────┴──────────┘
   Total:        ₦5,000     ₦5,000    ✓ Balanced

AFTER:
┌──────────────┬──────────┐    ┌──────────────┬──────────┐
│ Alice's Acc  │ ₦5,000   │    │  Bob's Acc   │ ₦8,000   │
└──────────────┴──────────┘    └──────────────┴──────────┘

VERIFICATION:
Total Money Before: ₦10,000 + ₦3,000 = ₦13,000
Total Money After:  ₦5,000  + ₦8,000 = ₦13,000  ✓ Balanced
```

## Concurrency Control - Race Condition Prevention

### Problem: Without Mutex

```
Initial Balance: ₦10,000

Thread 1                    Thread 2
  │                           │
  ├─ Read balance: ₦10,000   │
  │                           ├─ Read balance: ₦10,000
  ├─ Check: 10,000 >= 3,000  │
  │         ✓ OK              ├─ Check: 10,000 >= 2,000
  │                           │         ✓ OK
  ├─ Withdraw ₦3,000          │
  │  New: ₦7,000              ├─ Withdraw ₦2,000
  │                           │  New: ₦8,000
  └─ Write ₦7,000             │
                              └─ Write ₦8,000

PROBLEM: Final balance is ₦8,000
But should be ₦5,000 (10,000 - 3,000 - 2,000)
₦3,000 lost due to race condition!
```

### Solution: With Mutex

```
Initial Balance: ₦10,000

Thread 1                    Thread 2
  │                           │
  ├─ Lock(account) ────────┐  │
  │                        │  ├─ Lock(account) ← BLOCKED
  ├─ Read: ₦10,000        │  │    (waiting...)
  ├─ Check: OK            │  │
  ├─ Withdraw ₦3,000      │  │
  ├─ Write: ₦7,000        │  │
  └─ Unlock ──────────────┘  │
                              │
                              ├─ Lock acquired!
                              ├─ Read: ₦7,000
                              ├─ Check: OK
                              ├─ Withdraw ₦2,000
                              ├─ Write: ₦5,000
                              └─ Unlock

CORRECT: Final balance is ₦5,000 ✓
```

## Account State Machine

```
┌─────────────────────────────────────────────────────────┐
│                    ACCOUNT LIFECYCLE                    │
└─────────────────────────────────────────────────────────┘

    [New Account]
         │
         ▼
    ┌─────────┐
    │ ACTIVE  │ ◄──────────┐
    └────┬────┘            │
         │                 │ Reactivate
         │                 │
         ├─────────────────┴──────┐
         │                        │
         │ No Txn 90 Days         │ Frozen for
         ▼                        │ Investigation
    ┌──────────┐                 │
    │ DORMANT  │                 │
    └────┬─────┘                 │
         │                       │
         │ No Txn 1 Year         │
         ▼                       ▼
    ┌──────────┐           ┌──────────┐
    │  FROZEN  │ ◄─────────┤  FROZEN  │
    └────┬─────┘           └──────────┘
         │
         │ Close Request
         ▼
    ┌──────────┐
    │  CLOSED  │ (Terminal State)
    └──────────┘
```

## Transaction Status State Machine

```
    [New Transaction]
         │
         ▼
    ┌─────────┐
    │ PENDING │
    └────┬────┘
         │
         ├──────────────┬────────────┐
         │              │            │
         ▼              ▼            ▼
    ┌──────────┐  ┌─────────┐  ┌─────────┐
    │COMPLETED │  │ FAILED  │  │REVERSED │
    └──────────┘  └─────────┘  └─────────┘
         │              │
         └──────┬───────┘
                ▼
           (Terminal States)
```

## Interest Calculation Flow

```
Monthly Interest Calculation (e.g., 5% per annum)

┌────────────────────────────────────────────────┐
│ Step 1: Get Account Balance                   │
│         Balance = ₦100,000                     │
└──────────────────┬─────────────────────────────┘
                   │
                   ▼
┌────────────────────────────────────────────────┐
│ Step 2: Calculate Monthly Rate                │
│         Annual Rate = 5%                       │
│         Monthly Rate = 5% / 12 = 0.4167%       │
└──────────────────┬─────────────────────────────┘
                   │
                   ▼
┌────────────────────────────────────────────────┐
│ Step 3: Calculate Interest                    │
│         Interest = 100,000 × 0.004167          │
│                  = ₦416.70                     │
└──────────────────┬─────────────────────────────┘
                   │
                   ▼
┌────────────────────────────────────────────────┐
│ Step 4: Credit Interest to Account            │
│         New Balance = 100,000 + 416.70         │
│                     = ₦100,416.70              │
└──────────────────┬─────────────────────────────┘
                   │
                   ▼
┌────────────────────────────────────────────────┐
│ Step 5: Record Transaction                    │
│         Type: InterestCredit                   │
│         Amount: ₦416.70                        │
└────────────────────────────────────────────────┘

Compound Interest Effect (over 12 months):
Month 1:  ₦100,416.70
Month 2:  ₦100,835.13
Month 3:  ₦101,255.31
...
Month 12: ₦105,116.19 (5% gain)
```

## Batch Processing Flow (End of Day)

```
┌─────────────────────────────────────────────────────┐
│           END-OF-DAY BATCH PROCESSING               │
│            (Runs at 00:00 daily)                    │
└─────────────────────────────────────────────────────┘
    │
    ├─► Phase 1: Interest Calculation
    │     For each Savings Account:
    │       Calculate monthly interest
    │       Credit to account
    │
    ├─► Phase 2: Fee Processing
    │     For each Account:
    │       Check maintenance fee due
    │       Debit account
    │
    ├─► Phase 3: Standing Orders
    │     For each Active Standing Order:
    │       Check if due today
    │       Execute transfer
    │
    ├─► Phase 4: Dormancy Check
    │     For each Account:
    │       Check last transaction date
    │       If > 90 days: Mark DORMANT
    │
    ├─► Phase 5: Reconciliation
    │     Sum all debits
    │     Sum all credits
    │     Verify: Debits = Credits
    │     Generate variance report
    │
    └─► Phase 6: Reporting
          Generate:
            • Transaction summary
            • Account balances
            • Fee collection report
            • Regulatory reports (CBN)
            • Audit trail
```

## Security Layers

```
┌─────────────────────────────────────────────────────┐
│                 SECURITY ARCHITECTURE               │
└─────────────────────────────────────────────────────┘

Layer 1: Network Security
         │
         ├─► Firewall
         ├─► DDoS Protection
         ├─► SSL/TLS Encryption
         └─► VPN for Internal Access

Layer 2: Authentication
         │
         ├─► Multi-Factor Authentication (MFA)
         ├─► Biometric (Fingerprint, Face ID)
         ├─► PIN/Password
         └─► OTP (One-Time Password)

Layer 3: Authorization
         │
         ├─► Role-Based Access Control (RBAC)
         ├─► Transaction Limits
         ├─► IP Whitelisting
         └─► Device Recognition

Layer 4: Data Protection
         │
         ├─► Encryption at Rest
         ├─► Encryption in Transit
         ├─► PCI DSS Compliance
         └─► Data Masking

Layer 5: Monitoring & Audit
         │
         ├─► Real-time Fraud Detection
         ├─► Audit Logs (Immutable)
         ├─► Alerting System
         └─► Compliance Reporting
```

## Data Flow: Mobile Banking Transaction

```
   [Customer]
       │
       │ 1. Login with PIN + OTP
       ▼
   ┌────────┐
   │ Mobile │
   │  App   │
   └────┬───┘
        │ 2. Transfer Request
        │    (Encrypted)
        ▼
   ┌────────────┐
   │   API      │
   │  Gateway   │
   └────┬───────┘
        │ 3. Validate Token
        │    Check Rate Limits
        ▼
   ┌────────────────┐
   │ Banking System │
   │   (Core)       │
   └────┬───────────┘
        │ 4. Process Transaction
        │    • Validate
        │    • Debit/Credit
        │    • Log
        ▼
   ┌────────────┐
   │  Database  │
   │(PostgreSQL)│
   └────┬───────┘
        │ 5. Commit
        ▼
   ┌────────────┐
   │   Queue    │
   │  (Kafka)   │
   └────┬───────┘
        │ 6. Notify
        ▼
   ┌────────────┐      ┌────────────┐
   │   Email    │      │    SMS     │
   │  Service   │      │  Service   │
   └────────────┘      └────────────┘
        │                    │
        └───────┬────────────┘
                │ 7. Confirmation
                ▼
            [Customer]
```

## Rust Memory Model Benefits

```
Traditional Language (e.g., Java):
┌──────────────────────────────────────────┐
│  Account object in Heap Memory           │
│  ┌────────────────┐                      │
│  │  account_num   │ ← Reference 1        │
│  │  balance       │ ← Reference 2        │
│  │  ...           │ ← Reference N        │
│  └────────────────┘                      │
│                                          │
│  Problem: Multiple references can        │
│  modify simultaneously → Race conditions │
│  Solution: Manual synchronization        │
└──────────────────────────────────────────┘

Rust with Ownership:
┌──────────────────────────────────────────┐
│  Account object in Heap Memory           │
│  ┌────────────────┐                      │
│  │  account_num   │ ← SINGLE Owner       │
│  │  balance       │                      │
│  │  ...           │                      │
│  └────────────────┘                      │
│                                          │
│  Benefit: Only one owner can modify      │
│  Compiler enforces at compile-time       │
│  Zero runtime overhead                   │
└──────────────────────────────────────────┘

Rust with Arc<Mutex<>>:
┌──────────────────────────────────────────┐
│  Arc: Multiple owners allowed            │
│  Mutex: Enforces exclusive access        │
│                                          │
│  Thread 1 ──┐                            │
│             ├──► Mutex.lock() ──► Access │
│  Thread 2 ──┘      (Atomic)              │
│  Thread 3 ── Wait...                     │
│                                          │
│  Perfect for concurrent banking!         │
└──────────────────────────────────────────┘
```

## Key Takeaways

1. **Double-Entry Bookkeeping**: Every transaction balances
2. **Atomic Operations**: Transactions succeed or fail completely
3. **Concurrency Control**: Mutex prevents race conditions
4. **Audit Trail**: Every change is logged
5. **Type Safety**: Rust prevents entire classes of bugs
6. **Memory Safety**: No segfaults or memory leaks
7. **Validation**: Multiple layers of checks
8. **Error Handling**: Explicit error types
9. **State Machines**: Clear state transitions
10. **Batch Processing**: Automated daily operations
