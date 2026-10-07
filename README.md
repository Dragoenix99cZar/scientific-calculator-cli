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

```bash
scientific_calc.exe "PI"
PI = 3.141592653589793
```

```bash
scientific_calc.exe "PI" "_ + 1" "floor(_)"
PI = 3.141592653589793
3.141592653589793 + 1 = 4.141592653589793
floor(4.141592653589793) = 4
```

```bash
Scientific Calculator v0.2.0
Usage: scientific_calc.exe "<expression1>" "<expression2>" ...
     : use '#' to reference previous result

--- 1. Standard Arithmetic & Chaining ---
  scientific_calc.exe "2 + 3 * 4" "# * 2" -> 14, 28
  scientific_calc.exe "7 / 2" "floor(#)" "ceil(5.2 + #)" -> 3.5, 3, 9

--- 2. Scientific Notation & Digit Separators ---
  scientific_calc.exe "1E3 + 1E-3" -> 1000.001
  scientific_calc.exe "1_000_000 * 2" -> 2000000

--- 3. Advanced Math & Factorials ---
  scientific_calc.exe "abs(-42.5)" -> 42.5
  scientific_calc.exe "20!" -> 2432902008176640000
  scientific_calc.exe "mod(10, 3)" -> 1

--- 4. Trigonometry (Radians & Degrees) ---
  scientific_calc.exe "sin(pi / 2)" -> 1
  scientific_calc.exe "cos_d(60)" -> 0.5
  scientific_calc.exe "tan_d(45)" -> 1

--- 5. Multi-Argument Functions ---
  scientific_calc.exe "min(5, 2, 9, 1)" -> 1
  scientific_calc.exe "max(5, 2, 9, 1)" -> 9
  scientific_calc.exe "gcd(24, 36)" -> 12
  scientific_calc.exe "lcm(4, 6)" -> 12
  scientific_calc.exe "lerp(0.5, 10, 20)" -> 15

--- 6. Coordinate & Angle Conversions ---
  scientific_calc.exe "topol_r(3, 4)" -> 5
  scientific_calc.exe "topol_theta(0, 5)" -> 1.5707963267948966
  scientific_calc.exe "tocart_x(5, 60)" -> 2.50
  scientific_calc.exe "tocart_y(5, 90)" -> 5
  scientific_calc.exe "torad(180)" -> 3.141592653589793
  scientific_calc.exe "todeg(PI) -> 180"

--- 7. Base & Character Translations ---
  scientific_calc.exe "tobin(15)" "tooct(10)" -> 0b1111, 0o12
  scientific_calc.exe "tohex(255)" "toascii(97)" -> 0xFF, 'a'
```