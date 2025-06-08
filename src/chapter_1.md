# Chapter 1: Getting Started

Welcome to your first QuokkaSim simulation! In this chapter you will

1. Install Rust
2. Create a new Rust project  
3. Add QuokkaSim as a dependency  
4. Write and run a minimal "hello world" simulation  

---

## 1.1. Install Rust

Follow the instructs on the [Rust Lang website](https://www.rust-lang.org/tools/install) to install Rust on your operating system. These instructions will help you install:

- `rustup`, the Rust installer, and
- `cargo`, the Rust package manager

To check if installation is successful, use ``cargo -V`` to check which version of Cargo you have installed.

## 1.2. Create a new Rust project

If you don’t already have a project, open a terminal and run:

```bash
cargo new hello‐quokkasim
cd hello‐quokkasim
```

This creates a fresh binary crate with ``src/main.rs``.

## 1.3. Add QuokkaSim to Cargo.toml

To add QuokkaSim as a dependency, use ``cargo add quokkasim`` or add the following to your ``Cargo.toml`` file before running ``cargo fetch``:
```toml
[dependencies]
quokkasim = "0.1.0"
```

## 1.4. Write your first Simulation

In your ``main.rs`` file, paste in the following code at the top. This imports the required objects, and also creates our ``ComponentModel`` and ``ComponentLogger`` enums, which we will learn about later.

```rust,no_run
{{#include ../playground/src/bin/source_sink.rs:preamble}}
```

Next we create the individual interactive components of our simulation
<p align="center"><img src="images/source_sink_fig.png" alt="Figure 1: Simulation Overview" /><p>

`Stock 1` hold some quantity of material, which `Process` moves at specific times, into `Stock 2`. Add the following into the `main()` function to create these components, and to connect them together.

```rust,no_run
{{#include ../playground/src/bin/source_sink.rs:components}}
```

Next we'll add some ``Logger`` instances to report on what occurs during the simulation, and connect them to our Process and Stock components.

```rust,no_run
{{#include ../playground/src/bin/source_sink.rs:loggers}}
```

Then we create our `Simulation` object `sim`, which controls the progression of the simulation.

```rust,no_run
{{#include ../playground/src/bin/source_sink.rs:sim}}
```

We send and initialisation events, tell our simulation to run for an hour, and write the resulting logs to CSV files.

```rust,no_run
{{#include ../playground/src/bin/source_sink.rs:run}}
```

Our `main.rs` file is now complete (or refer to the Full Code below if you think you're missing something).

Use `cargo run` to run the simulation, and we have our logs in the `outputs/source_sink` directory!

## 1.5. Exercises

Want to start playing around immediately? Here are some ideas of things you can try before moving on with the rest of the book!

- 5 minutes is simulated to begin with. What if we simulate for longer?
- The initial simulation begins with signicant stock in `Stock 1`. What happens if it starts empty?
- What if instead of a sink removing material from the system, we make `Stock 2` bigger and see how long it takes to fill up?
- What if we add an additional sink that takes directly from `Stock 1`, or an addition source that feeds directly into `Stock 2`?
- What if there were no source or sink, but instead an additional process from `Stock 2` into `Stock 1`?
- What if we want to have logs for `Source` and `Sink` save into their own log files?

---

## Full Code
```rust,no_run
{{#include ../playground/src/bin/source_sink.rs:all}}
```