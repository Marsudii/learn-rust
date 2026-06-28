# 🦀 Learn Rust Basic

A simple Rust learning project for understanding the fundamentals of the Rust programming language.

This repository contains basic Rust examples such as:

* Printing output
* Variables
* Data types
* Functions
* Modules
* Basic project structure
* More Rust fundamentals coming soon

---

## 📌 About This Project

**Learn Rust Basic** is a beginner-friendly project created to learn Rust step by step.

The goal of this project is to build a strong foundation in Rust by writing simple examples and organizing them into clean modules.

---

## 🛠️ Prerequisites

Before running this project, make sure Rust is installed on your machine.

Rust installation includes:

* `rustc` — Rust compiler
* `cargo` — Rust package manager and build tool
* `rustup` — Rust toolchain manager

Official Rust installation:

```txt
https://www.rust-lang.org/tools/install
https://rustup.rs/
```

---

## 💻 Install Rust

### Windows

#### Option 1: Install from Rustup

1. Open this URL in your browser:

```txt
https://rustup.rs/
```

2. Download `rustup-init.exe`
3. Run the installer
4. Follow the installation instructions
5. Restart your terminal

Verify installation:

```powershell
rustc --version
cargo --version
rustup --version
```

#### Option 2: Windows Subsystem for Linux

If you use WSL, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Verify installation:

```bash
rustc --version
cargo --version
rustup --version
```

---

### macOS

Install Rust using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Verify installation:

```bash
rustc --version
cargo --version
rustup --version
```

---

### Linux

Install Rust using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Verify installation:

```bash
rustc --version
cargo --version
rustup --version
```

---

## 📁 Project Structure

```txt
learn-rust-basic/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    └── basic/
        ├── mod.rs
        └── print.rs
```

---

## 🚀 How to Run

Clone this repository:

```bash
git clone https://github.com/your-username/learn-rust-basic.git
```

Go to the project directory:

```bash
cd learn-rust-basic
```

Run the project:

```bash
cargo run
```

---

## 🧪 Useful Cargo Commands

Check if the code compiles:

```bash
cargo check
```

Build the project:

```bash
cargo build
```

Run the project:

```bash
cargo run
```

Build for release:

```bash
cargo build --release
```

Format the code:

```bash
cargo fmt
```

Run Rust linter:

```bash
cargo clippy
```

---

## 📚 Learning Contents

| No | Topic          | Status |
| -: | -------------- | :----: |
|  1 | Print output   |    ✅   |
|  2 | Variables      |   🚧   |
|  3 | Data types     |   🚧   |
|  4 | Functions      |   🚧   |
|  5 | Modules        |   🚧   |
|  6 | Conditionals   |    ⏳   |
|  7 | Loops          |    ⏳   |
|  8 | Ownership      |    ⏳   |
|  9 | Borrowing      |    ⏳   |
| 10 | Structs        |    ⏳   |
| 11 | Enums          |    ⏳   |
| 12 | Error handling |    ⏳   |

Legend:

```txt
✅ Done
🚧 In Progress
⏳ Coming Soon
```

---

## 🧩 Example Module

Example file:

```txt
src/basic/print.rs
```

```rust
pub fn print() {
    print!("PRINT NO ENTER");
    println!("PRINT WITH ENTER");
}
```

Register module:

```txt
src/basic/mod.rs
```

```rust
pub mod print;
```

Call function from `main.rs`:

```rust
mod basic;

fn main() {
    basic::print::print();
}
```

---

## 🎯 Project Goal

This project is used as a personal Rust learning journey.

Main goals:

* Understand Rust syntax
* Practice Rust fundamentals
* Learn how Rust modules work
* Build clean and organized Rust examples
* Prepare for more advanced Rust topics

---

## 🔥 Why Rust?

Rust is a modern systems programming language focused on:

* Performance
* Memory safety
* Concurrency
* Reliability
* Zero-cost abstractions

Rust is commonly used for:

* CLI tools
* Backend services
* Systems programming
* WebAssembly
* Embedded systems
* High-performance applications

---

## 📖 References

* Rust Official Website: https://www.rust-lang.org/
* Rustup Installer: https://rustup.rs/
* The Rust Book: https://doc.rust-lang.org/book/
* Cargo Book: https://doc.rust-lang.org/cargo/

---

## 👨‍💻 Author

Created by **your-name-here**

This repository is part of my journey to learn Rust from basic to advanced.

---

## ⭐ Support

If this repository helps you learn Rust, feel free to give it a star.

```txt
Keep learning. Keep building. Keep shipping.
```
