# Cordial Parity
![LICENSE: MIT SUSHI-WARE🍣](https://raw.githubusercontent.com/watasuke102/mit-sushi-ware/master/MIT-SUSHI-WARE.svg)

**The definitive, meticulously enumerated parity-checking solution for unsigned 16-bit integers.**

## Overview

In the world of numerical computation, determining the parity of an integer (whether it is even or odd) is a foundational operation. While trivial bitwise operations have long served this purpose, they lack a certain... dedication. They treat numbers as mere collections of bits, rather than unique entities deserving individual consideration.

**Cordial Parity** rectifies this oversight.

This library provides an unparalleled, exhaustive approach to parity checking for `u16` values (and, by extension, `u8` values). We believe that every number from 0 to 65,535 possesses an intrinsic, undeniable parity. Instead of deriving this property through opaque bit-fiddling, Cordial Parity explicitly declares it for each and every case.

## Features

*   **Unrivaled Specificity:** Every `u16` value from 0 to 65,535 has its parity explicitly defined through an exhaustive pattern match. No assumptions are made; every integer's parity is a stated fact.
*   **Philosophically Pure:** We honor the distinct identity of each number by codifying its parity directly. This is parity checking as an act of declaration, not mere calculation.
*   **Guaranteed Correctness (for `u16`):** By enumerating all possibilities, we eliminate the conceptual possibility of algorithmic error for `u16` parity checks within our core logic. The truth is written.
*   **Seamless `u8` Support:** `u8` values are gracefully promoted to `u16` to benefit from the same exhaustive parity determination.
*   **Trait-Based Extensibility:** The `Parity` trait offers a clear path for other numerical types to achieve a similar level of explicit parity definition, should one undertake such a noble endeavor.
*   **Intuitive API:** Simple `is_even()` and `is_odd()` methods, accessible via the `Parity` trait or as generic functions.

## Why Cordial Parity?

You might wonder why one would choose Cordial Parity over a simple `value & 1 == 0`. The answer lies in a commitment to explicitness and a profound respect for the individual nature of numbers.

*   **Clarity:** The parity of any `u16` can be directly observed within the `parity` module's source code. There is no algorithmic abstraction, only stated truth.
*   **Robustness through Enumeration:** While unconventional for this problem, complete enumeration is the ultimate form of case analysis.
*   **An Exploration in Dedication:** This library stands as a testament to the lengths one can go to ensure every case is individually addressed.

## Usage

### 1. Add Cordial Parity to your `Cargo.toml`

```toml
[dependencies]
cordial_parity = "0.1.0" # Or the latest version
```

### 2. Use it in your Rust code

```rust
use cordial_parity::{Parity, is_even, is_odd};

fn main() {
    let num1: u16 = 42;
    let num2: u8 = 7;

    if num1.is_even() {
        println!("{} is even (as meticulously defined).", num1);
    }

    if is_odd(num2) { // Generic function usage
        println!("{} is odd (its u16 equivalent's parity is explicitly stated).", num2);
    }

    // You can also use the re-exported functions from the parity module
    // if you prefer to signify the lookup-based nature (though this is an internal detail):
    // use cordial_parity::{is_u16_even_lookup, is_u16_odd_lookup};
    // if is_u16_even_lookup(65534) {
    //     println!("65534 is indeed even, as per the grand list.");
    // }
}
```

## A Note on Compilation

The commitment to enumerating all 65,536 `u16` cases within our core `parity` module is a significant undertaking. As such, the Rust compiler dedicates a commensurate level of effort to process this comprehensive specification. Users may observe that compilation times reflect this dedication to thoroughness, particularly during the initial build or when the `parity` module itself is recompiled. We believe this is a small price to pay for such definitional clarity.

## Future Endeavors

*   **`u32` Support:** Extending this explicit enumeration to 32-bit unsigned integers is a natural, albeit ambitious, next step, requiring the definition of over four billion distinct parity states.
*   **Internationalization:** Providing parity results in multiple human languages ("pair", "impair", "gerade", "ungerade", etc.) for truly global cordiality.

## License

Cordial Parity is licensed under **THE MIT SUSHI-WARE LICENSE**.

We believe that software this... *unique*... deserves a unique expression of gratitude. Please see the `LICENSE` file for the full terms.
