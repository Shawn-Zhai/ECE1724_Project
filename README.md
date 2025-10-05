# ECE1724 Performant Software Systems with Rust  
## Course Project – Personal Finance Tracker

**Team Members**  
- **Tony Cai** — 1012411123  
- **Charlie Yin** — 1006163679  
- **Shawn Zhai** — 1006979389  

---

## Motivation

Managing personal finances effectively is a universal challenge, yet most existing tools are either **overly simplistic** (e.g., spreadsheets, single-category trackers) or **locked behind proprietary paywalls** that emphasize mobile or web experiences. For developers and power users who spend much of their time in the terminal, these tools are often either too limited or too rigid to fit into personalized workflows.

While terminal-based finance tools do exist in other ecosystems (e.g., **Ledger**, **hledger** in Haskell), there is currently **no performant, customizable, open-source personal finance tracker built in Rust** that combines:

- a **fast, reliable HTTPS backend** for secure data storage and multi-device use, and  
- a **responsive TUI (text user interface)** for efficient day-to-day interaction.

This gap presents both a practical opportunity and a pedagogical challenge that fits ECE1724 perfectly.

---

### Why Rust?

Rust is particularly well-suited to this project because of its:

- **Memory safety guarantees**, which ensure financial data structures and transaction handling code are free of undefined behavior and memory corruption issues.  
- **Performance**, allowing the backend to process transactions and queries quickly without garbage collection pauses.  
- **Strong type system**, which lets us encode domain invariants directly in types (e.g., preventing negative income, ensuring category splits sum to transaction totals).  
- **Concurrency primitives** (e.g., async/await, Tokio runtime) that make it possible to build a backend that scales well even with multiple simultaneous requests.

Additionally, the Rust ecosystem offers high-quality libraries like **Axum** (modern web framework), **SQLx** (async SQL toolkit), and **Ratatui** (powerful TUI library). However, there is currently no cohesive example project that demonstrates how to integrate these libraries into a complete, performant application. By filling this gap, our project contributes meaningfully to the community while giving us experience with real-world Rust systems programming.

---

### Project Goals

This project is driven by three complementary goals:

- **Developer Empowerment** — Build a tool tailored for developers and terminal power users who prefer open-source, offline-first workflows with optional syncing.  
- **Ecosystem Contribution** — Demonstrate how modern Rust frameworks can be composed into a secure, performant CLI + backend system, filling a notable gap in existing examples.  
- **Practical Usefulness** — Deliver a real-world application that our team can actually use to manage income, expenses, and budgets across multiple accounts efficiently.

By the end of the course, we aim to produce a fully functional, polished Rust application that can be **self-hosted**, **extended**, and potentially **open-sourced** as a community project.

---

## Objective and Key Features

### **Overall Objective**

The goal of this project is to **design and implement a performant, extensible personal finance tracker** with a command-line interface. The tracker will communicate with a backend server over HTTPS to manage users’ **income, expenses, categories, accounts, and budgets**, and will support **complex transactions** and **reconciliation workflows** that go beyond simple spreadsheets.

This tool should allow a user to log transactions quickly, categorize them flexibly, and review their financial history or budgets in an efficient TUI environment — all while maintaining strong guarantees about **data integrity** and **system performance**.

---

### **Key Features**

#### 1. Backend Database with HTTPS API

- Built using **Axum** or **Actix Web**, exposing a secure RESTful API over HTTPS.  
- Uses **PostgreSQL** (via SQLx) for async data access.  
- HTTPS ensures sensitive data remains secure even across networks.  
- Enables clean separation between frontend and backend for future multi-user support.

---

#### 2. Transaction Logging

- Users can log both **income** and **expenses**, with timestamps and metadata.  
- Transactions can be **edited or deleted**.  
- Each entry includes category and account metadata.  
- Consistent logging is the foundation for reconciliation, reporting, and budgeting.

---

#### 3. Categories & Multi-Category Transactions

- Users can define **custom spending categories** (e.g., “Groceries,” “Rent,” “Utilities”).  
- Supports **complex transactions split across multiple categories** (e.g., grocery trip with food + household).  
- Rust’s type system ensures the sum of splits matches the total, preventing inconsistencies.

---

#### 4. Multiple Accounts

- Supports different account types: **checking**, **savings**, and **credit card**.  
- Allows tracking of **transfers between accounts** and reconciliation of balances.

---

#### 5. Reconciliation

Ensuring that all transactions between a user’s different accounts (e.g., Cash, Checking, Credit Card, Savings) are **consistent and balanced**:

| Category Goal  | Description                              |
|---------------|-------------------------------------------|
| **Concurrency** | Prevent simultaneous writing              |
| **Consistency** | Ensure user terminal matches backend     |
| **Validation**  | Prevent invalid data                     |
| **Sync Safety** | Terminal and backend states remain aligned |

---

#### 6. Budgeting & Financial Reports (Optional)

If time permits, we will add **budgeting tools** (e.g., monthly category budgets) and **reports** that visualize financial trends.  
These could include bar charts of monthly spending or line charts of income over time, rendered directly in the TUI.

---

#### 7. Text-Based User Interface (TUI)

Built with **Ratatui**, optimized for keyboard navigation and responsiveness. Users can:

- View recent transactions and reports  
- Add/edit transactions efficiently  
- Switch between account and category views

---

## Novelty in the Rust Ecosystem

Although Axum, SQLx, and Ratatui are individually mature, **no existing Rust project integrates them into a single finance tracker with HTTPS synchronization and multi-account support**.  
This project fills that gap, serving as both a **useful tool** and an **instructive example** of performant Rust systems programming.

---

## Tentative Plan

We are a team of three members. Our timeline is designed to deliver a **functional core system early**, then iterate on optional features and performance improvements.

| Task Area                     | Description                                                                                                      | Responsible Member |
|-------------------------------|-------------------------------------------------------------------------------------------------------------------|---------------------|
| **Backend & Database**        | - Set up an Axum or Actix Web server with HTTPS.<br>- Define REST API endpoints (transactions, accounts, categories).<br>- Implement PostgreSQL schema and migrations using SQLx.<br>- Ensure proper error handling and performance benchmarks. | Tony Cai           |
| **TUI Frontend**              | - Build CLI interface using Ratatui.<br>- Implement navigation, transaction entry forms, and report views.<br>- Handle input validation and rendering efficiently. | Charlie Yin        |
| **Integration & Advanced Features** | - Implement reconciliation logic and multi-category transaction handling.<br>- Integrate frontend and backend using async HTTPS requests.<br>- Develop budgeting and reporting features if time permits.<br>- Conduct end-to-end testing and profiling. | Shawn Zhai         |

---

## Implementation Plan (High-Level)

### Week 1
- Set up repository structure and CI.  
- Initialize backend skeleton (Axum) and TUI skeleton (Ratatui).  
- Agree on API schema and database schema.  
- Assign detailed responsibilities and finalize interfaces.

### Week 2
- Implement core backend CRUD for transactions and accounts.  
- Develop basic TUI for viewing and adding transactions.  
- Begin integration testing between CLI and backend over HTTP.

### Week 3
- Add support for multi-category transactions and reconciliation.  
- Improve TUI navigation and error handling.  
- Add HTTPS and authentication stubs for future security.  
- Conduct performance profiling and optimize slow paths.

### Week 4
- Implement budgeting and financial reports if time permits.  
- Perform comprehensive integration testing.  
- Polish documentation, clean code, and prepare for final demo.

---

We believe this plan is **feasible within the project duration**. The core system (backend + TUI + basic transaction handling) is scoped to be achievable in three weeks, leaving time for polish and optional features.
