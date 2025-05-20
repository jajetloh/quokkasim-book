# The QuokkaSim Book

Welcome to the **QuokkaSim Book** – your guide to learning and mastering QuokkaSim, the Rust-based discrete-event simulation framework.

> **Note:**  
> QuokkaSim is under active development. If you find typos or missing features, let JJ know!

---

## What is QuokkaSim?

QuokkaSim is a high-performance, event-driven simulation framework written in Rust, on top of the [NeXosim](https://github.com/asynchronics/nexosim) simulation engine.

---

## Who should read this book?

This book is written for:

1. You
2. Some other people too

---

## FAQ

**Why does QuokkaSim use Rust?**

- Building QuokkaSim in Rust means:
    - QuokkaSim can be free and open source under the MIT License.
    - QuokkaSim is highly performant (thanks to the [NeXosim](https://github.com/asynchronics/nexosim) library).
    - Simulations built in QuokkaSim can benefit from the type safety, memory safety, and explicit error handling that is fundamental to Rust. This strictness minimises issues that arise as a simulation model scales and increases in complexity.