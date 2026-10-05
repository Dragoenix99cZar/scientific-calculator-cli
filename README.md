```text
▄▀▀▀▄        ▀               █    ▀   ▄▀▄  ▀             ▄▀▀▀▄       ▀█       
▀▄▄▄  ▄▀▀▀▄ ▀█  ▄▀▀▀▄ █▄▀▀▄ ▀█▀▀ ▀█  ▄█▄  ▀█  ▄▀▀▀▄      █     ▀▀▀▄   █  ▄▀▀▀▄
▄   █ █   ▄  █  █▀▀▀▀ █   █  █ ▄  █   █    █  █   ▄      █   ▄ ▄▀▀█   █  █   ▄
 ▀▀▀   ▀▀▀  ▀▀▀  ▀▀▀  ▀   ▀   ▀  ▀▀▀ ▀▀▀  ▀▀▀  ▀▀▀        ▀▀▀   ▀▀ ▀ ▀▀▀  ▀▀▀ 
```

```text
========================================================================================
  A lightning-fast, zero-dependency, memory-efficient scientific CLI calculator 
  engineered in Rust.
========================================================================================
```

---

## 1. Ideation

Most CLI tools or calculators rely on heavy third-party regex engines or bloated runtimes. This project was conceptualized to build a **micro-footprint, high-performance scientific calculator** from scratch.

By leveraging pure standard-library character iterators and the **[Shunting-Yard algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm)**, it achieves sub-millisecond execution times, zero external package bloat, and a tiny binary footprint optimized for size (`opt-level = "z"`). It also features stateful history tracking using `_` to chain calculations seamlessly across command arguments.

---

## 2. Features

* **Zero External Dependencies:** Built entirely using Rust's `std` library, ensuring rapid compilation and a minimal binary size.


* **Scientific Operations:** Native support for trigonometric functions (`sin`, `cos`, `tan`), roots (`sqrt`), logarithms (`ln`, `log10`, `log2`), exponentials (`exp`), and rounding modifiers (`floor`, `ceil`).


* **Constants Mapping:** Built-in definitions for mathematical constants (`PI`, `E`, `TAU`).


* **History Substitution (`_`):** Track state across multiple quoted command-line arguments dynamically, allowing you to feed the result of the previous expression into the next.

---

## 3. Tech Used

* **Language:** Rust (Edition 2024)
* **Core Logic:** Manual lightweight tokenizer, Shunting-Yard infix-to-postfix parser, and stack-based postfix evaluator.


* **Compiler Optimizations:** Link-Time Optimization (`lto = true`), size-focused code generation (`opt-level = "z"`), and symbol stripping (`strip = true`).

---

## 4. Setup & Installation

### 4.1. Initialize Project

Clone or create your project directory and configure `Cargo.toml`:

```toml
[package]
name = "scientific_calc"
version = "0.1.0"
edition = "2024"

[dependencies]
# Zero external dependencies for maximum speed and tiny file size!

[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization for better dead-code elimination
codegen-units = 1   # Maximizes optimization efficiency
panic = "abort"     # Removes unwinding bloat
strip = true        # Strips symbols/debug info from the final binary
```

### 4.2. Build Release Binary

Compile an optimized, stripped production binary for Windows 11:

```bash
cargo build --release
```

Your compiled executable will be located at `target/release/scientific_calc.exe`.

---

## 5. Usage

Run expressions directly from your command line. Expressions **must be enclosed in quotes**, and sequential arguments can chain history data using the underscore character `_`.

```bash
scientific_calc.exe "<expression1>" "<expression2>" ...
```

---

## 📋 Examples

### Example 1: Basic Arithmetic & History Chaining



```bash
scientific_calc.exe "2 + 3" "_ * 2"
2 + 3 = 5
5 * 2 = 10
```

### Example 2: Scientific Functions & Rounding

```bash
scientific_calc.exe "7 / 2" "floor(_)" "ceil(5.2 + _)"
7 / 2 = 3.5
floor(3.5) = 3
ceil(5.2 + 3) = 9
```

### Example 3: Trigonometry & Constants



```bash
scientific_calc.exe "sin(PI / 2)" "exp(1) * _"
sin(1.5707963267948966) = 1
2.718281828459045 * 1 = 2.718281828459045
```

```console
scientific_calc.exe "PI"
PI = 3.141592653589793
```

```console
scientific_calc.exe "PI" "_ + 1" "floor(_)"
PI = 3.141592653589793
3.141592653589793 + 1 = 4.141592653589793
floor(4.141592653589793) = 4
```
