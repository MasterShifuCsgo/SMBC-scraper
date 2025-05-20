# How to Use

## 1. Install Rust and Required Tools

### 🔧 Step 1: Install [Microsoft Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

> ⚠️ Only install the **Build Tools**.  
> During installation, make sure to select:
>
> - ✅ **C++ build tools**
> - ✅ **MSVC v142+** (or latest available)
> - ✅ **Windows 10 SDK** or **Windows 11 SDK**

➡️ You can ignore everything else.

---

### 🦀 Step 2: Install [Rust](https://www.rust-lang.org/tools/install)

This will install the following tools:

- `rustup` — Rust installer and version manager  
- `cargo` — Rust's build system and package manager  
- `rustc` — The Rust compiler

---

## 2. Clone the Repository and Run the Project

Open a terminal (Command Prompt, PowerShell, or Git Bash), then run:

```bash
git clone https://github.com/your-username/your-repo-name.git
cd your-repo-name
cargo run
```

> ⚠️ The first `cargo run` may take a while as it compiles dependencies.

Once it's built, you can open `src/main.rs` in your code editor to configure or customize the logic as needed.
