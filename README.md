# 🔍 Rust Port Scanner

A fast, efficient, and fully multithreaded **TCP port scanner** built in Rust. Perfect for developers, sysadmins, and cybersecurity enthusiasts who need to quickly probe for open ports on any IPv4/IPv6 target.

Designed for **speed** and **simplicity**, this scanner covers all 65,535 ports using customizable parallelism.

---

## ✨ Features

✅ Ultra-fast port scanning using multithreading
✅ Lightweight – built with only the Rust standard library
✅ IPv4 & IPv6 support
✅ Configurable thread pool with `-j` flag
✅ Clean and readable CLI interface
✅ Zero dependencies – portable & cross-platform

---

## 📦 Clone & Run Locally

1. **Clone the repository**

```bash
git clone https://github.com/prajwal-1703/port_snipper_rust.git
cd port_snipper_rust/ip_snipper
```

2. **Build the project**

```bash
cargo build --release
```

3. **Run the scanner**

```bash
cargo run -- -j 1000 192.168.1.1
```

> 💡 Make sure you `cd` into the `ip_snipper` directory before running `cargo` commands. The root folder doesn't contain a `Cargo.toml`.

---

## 🚀 Getting Started

### 🔧 Build the Project

```bash
cargo build --release
```

### 🏃‍♂️ Run the Scanner

#### Default (4 threads)

```bash
cargo run -- 127.0.0.1
```

#### With custom threads (e.g. 1000 threads)

```bash
cargo run -- -j 1000 192.168.1.1
```

#### Display Help

```bash
cargo run -- -h
```

> 💡 Tip: The `--` after `cargo run` is required to forward flags to your Rust binary.

---

## 📘 Command Line Arguments

| Argument            | Description                                 |
| ------------------- | ------------------------------------------- |
| `<ip>`              | IP address (IPv4 or IPv6) to scan           |
| `-j <threads> <ip>` | Use a custom number of threads for scanning |
| `-h` or `-help`     | Display usage/help message                  |

---

## 🔍 Sample Output

```
Scanning with 1000 threads...
....................................................
Port 21 is open
Port 23 is open
Port 53 is open
Port 80 is open
Port 443 is open
Port 52881 is open
Scan complete.
```

---

## 📁 Project Structure

* `main.rs` – Main entry point
* `Arguments` – CLI argument parser and validator
* `scan()` – Multithreaded port scanner logic

---

## 📄 Word from the Author
Free to use, modify, and distribute.

---

## 👨‍💻 Author

Built with ⚙️ systems-level love using [Rust](https://www.rust-lang.org/) 🦀
Ready for contribution, feedback, or collaboration!
