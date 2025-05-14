# Chapter 1: Getting Started

Welcome to your first QuokkaSim simulation! In this chapter you will

1. Create a new Rust project  
2. Add QuokkaSim as a dependency  
3. Write and run a minimal "hello world" simulation  

---

## 1.1. Create a new Rust project

If you don’t already have a project, open a terminal and run:

```bash
cargo new hello‐quokkasim
cd hello‐quokkasim
```

This creates a fresh binary crate with ``src/main.rs``.

## 1.2. Add QuokkaSim to Cargo.toml

To add QuokkaSim as a dependency, use ``cargo add quokkasim`` or add the following to your ``Cargo.toml`` file before running ``cargo fetch``:
```toml
[dependencies]
quokkasim = "0.0.1"
```

## 1.3. Write your first Simulation

In your ``main.rs`` file, paste in the following code at the top. This imports the required objects, and also creates our ``ComponentModel`` and ``ComponentLogger`` enums, which we will learn about later.

```rust,no_run
{{#include ../playground/src/bin/example_0.rs:preamble}}
```

Next we create the individual interactive components of our simulation
<p align="center"><img src="images/fig-01.png" alt="Figure 1: Simulation Overview" /><p>

`Stock1` hold some quantity of material, which `Process1` moves at specific times, into `Stock2`. Add the following into the `main()` function to create these components, and to connect them together.

```rust,no_run
{{#include ../playground/src/bin/example_0.rs:components}}
```

Next we'll add some ``Logger`` instances to report on what occurs during the simulation, and connect them to our Process and Stock components.

```rust,no_run
{{#include ../playground/src/bin/example_0.rs:loggers}}
```

Then we create our `Simulation` object `sim`, which controls the progression of the simulation.

```rust,no_run
{{#include ../playground/src/bin/example_0.rs:sim}}
```

We send and initialisation events, tell our simulation to run for an hour, and write the resulting logs to CSV files.

```rust,no_run
{{#include ../playground/src/bin/example_0.rs:run}}
```

Our `main.rs` file is now complete (or refer to the Full Code below if you think you're missing something).

Use `cargo run` to run the simulation, and we have our logs in the `outputs/example_0` directory!

---

## Full Code

```rust,no_run
{{#include ../playground/src/bin/example_0.rs:all}}
```