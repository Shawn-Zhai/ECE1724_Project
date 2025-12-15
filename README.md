# ECE1724 Performant Software Systems with Rust  
# Video Slide Presentation: 
# Video Demo: 
A short demo showcasing the system and its core workflows is available here:

▶️ [Watch the demo video](https://drive.google.com/file/d/1L6DFaM4mtiISFeKDEs6_DEqHNVvXf-6D/view?usp=drive_link)

---

## Final Project Report – Personal Finance Tracker

**Team Members**  
- **Tony Cai** — 1012411123      t.cai@mail.utoronto.ca
- **Charlie Yin** — 1006163679   zhiqiang.yin@mail.utoronto.ca
- **Shawn Zhai** — 1006979389    shawn.zhai@mail.utoronto.ca

---

## Motivation

Personal finance management is a fundamental yet often underserved problem in everyday software tools. While numerous finance applications exist, many of them rely on cloud-based services, require persistent internet connectivity, or store sensitive financial data on third-party servers. These design choices reduce transparency and may raise privacy concerns for users who prefer local control over their data.

Our team was motivated to design and implement a **local-first, terminal-based personal finance tracker** that emphasizes simplicity, transparency, and user ownership of data. By avoiding graphical interfaces and external services, we aimed to build a lightweight tool that is portable, efficient, and suitable for a wide range of environments, including servers and minimal systems.

From a technical perspective, this project also served as an opportunity to explore Rust as a language for building **complete, user-facing applications**, rather than only libraries or low-level system components. We wanted to demonstrate how Rust’s strong guarantees around memory safety, concurrency, and correctness can be applied to a realistic application with persistent state and interactive user input.

Finally, the project aligns closely with the goals of **ECE1724: Performant Software Systems**, allowing us to apply safe systems design, modular architecture, performance awareness, and reproducibility in a concrete setting.

---

## Objectives

The objectives of this project are:

1. Build a functional personal finance tracking application that supports recording income and expenses.
2. Organize financial data using categories to improve clarity and analysis.
3. Maintain a clear separation between backend logic and frontend interaction.
4. Leverage Rust’s ownership model and type system to ensure correctness and safety.
5. Provide an intuitive terminal-based user interface.

---

## Features

### Transaction Management

The application allows users to create and manage financial transactions representing both income and expenses. Each transaction includes:

- Transaction type (income or expense)
- Monetary amount
- Category
- Optional description
- Date or timestamp

This structured representation forms the foundation of the application.

---

### Categorization of Financial Data

Transactions can be assigned to categories such as food, rent, transportation, or salary. Categorization enables users to:

- Analyze spending patterns
- Identify major expense sources
- Maintain organized financial records

---

### Terminal User Interface (TUI)

The project provides a text-based user interface that runs entirely in the terminal. The TUI:

- Presents menus and prompts for interaction
- Displays transaction data in a readable format
- Does not rely on any graphical environment

This improves portability and allows the application to run on minimal systems.

---

### Local Persistent Storage

All financial data is stored locally on the user’s machine. This design:

- Preserves privacy
- Enables offline use
- Avoids reliance on external services

Data persistence is handled transparently by the backend.

---

### Modular and Extensible Design

The project is organized into backend and frontend components with clear interfaces. This design improves maintainability and makes it easier to add future features such as summaries or reports.

---

## User’s / Developer’s Guide

### User Guide

From the repo root, start the backend (cargo run -p backend), then in another terminal start the TUI frontend (cargo run -p frontend). In the TUI, follow the menu to add income/expense transactions, choose or type categories, optionally add descriptions and dates, review the transaction list, and use edit/delete when available; changes save automatically, and you can quit via the on-screen exit option (then stop the backend with Ctrl+C).

---

### Developer Guide

Backend code lives in backend/ and TUI code in frontend/. Build or run with Cargo (cargo build, cargo run -p backend, cargo run -p frontend); cargo test runs tests if present. Integrate new features by extending backend services (data models, storage, APIs) and updating frontend views/commands to call them. Keep category/type validation in the backend, preserve the existing interfaces between the two crates, and run both processes to verify end-to-end behavior.

---

## Reproducibility Guide

It is assumed that **Rust and Cargo are already installed** on the system.

---

### Step 1: Clone the Repository
git clone https://github.com/Shawn-Zhai/ECE1724_Project.git
cd ECE1724_Project

### Step 2: Run the Backend Server
cargo run -p backend

### Step 3: Run the Frontend
cargo run -p frontend

---

## Contributions by Each Team Member

---

### Tony Cai

- Designed and implemented **transaction edit and delete workflows**, enabling full lifecycle management of financial records.
- Added and refined a **backend service layer**, improving modularity and interaction between system components.
- Worked closely with Shawn to ensure that new backend features integrated cleanly with existing transaction logic.
- Participated in **feature testing and validation**, identifying edge cases and usability issues.
- Contributed to iterative refinement of the application’s behavior based on testing feedback.
- Assisted with debugging and stabilization during later stages of development.

### Charlie Yin

- Developed the **initial draft of the terminal user interface**, defining how users navigate and interact with the application.
- Implemented early frontend components that shaped menu layout, interaction flow, and data presentation.
- Iteratively refined the frontend structure during early development, providing a usable base for further expansion.
- Collaborated with other team members to align frontend behavior with backend capabilities.
- Participated in **testing and feedback cycles**, helping identify UI limitations and integration issues.
- Supported final integration and polishing by validating frontend behavior against backend changes.

### Shawn Zhai

- Implemented the **core backend transaction model**, including transaction creation, validation, and integration logic.
- Added **persistent storage and data synchronization mechanisms**, ensuring consistency across application runs.
- Designed and implemented **account management features**, including account creation, deletion, and transfer logic.
- Integrated the backend with the TUI by exposing backend APIs to the frontend layer.
- Improved system robustness by enforcing **business rules** (e.g., preventing invalid or negative transaction amounts).
- Performed extensive **debugging, refactoring, and code cleanup**, resolving import issues and improving maintainability.
- Coordinated feature integration across team contributions and ensured overall system stability.

---

## Lessons Learned and Concluding Remarks

This project provided valuable experience in designing and implementing a complete software system in Rust. Key lessons learned include:

- Rust’s ownership and type system promote safer and more deliberate software design.
- Separating backend logic from frontend presentation simplifies debugging and extension.
- Terminal-based applications can be efficient, portable, and user-friendly when designed carefully.
- Clear documentation and explicit setup instructions are essential for reproducibility.

Overall, the project successfully met its objectives and demonstrated how Rust can be used to build performant, reliable, and user-facing software systems. The experience reinforced the core principles taught in ECE1724 and provided a strong foundation for future systems-level development.

---

## Course Project Proposal – Personal Finance Tracker

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
