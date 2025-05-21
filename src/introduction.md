# The QuokkaSim Book

Welcome to the **QuokkaSim Book** – your guide to learning and mastering QuokkaSim, the Rust-based discrete-event simulation framework.

> **Note:**  
> QuokkaSim is under **very active** development. Let JJ know if:
>
> - You find mistakes, typos or missing information
> - You have ideas to contribute, or want to contribute in a more hands-on capacity
> - You're unsure if QuokkaSim is suitable for your use case. If it's not, let's discuss how it can be!
>
> If you are reading this, you probably already know how to contact me 😉 - JJ

---

## What is QuokkaSim?

QuokkaSim is a high-performance, event-driven simulation framework written in Rust, on top of the [NeXosim](https://github.com/asynchronics/nexosim) simulation engine.

QuokkaSim is also:
- Open source
- Permissively licensed (MIT License)
- Memory-safe
- Highly performant, via the NeXosim crate
- Accessible to new simulation modellers and new Rust developers

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