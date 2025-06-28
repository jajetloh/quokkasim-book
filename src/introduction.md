# The QuokkaSim Book

Welcome to the **QuokkaSim Book** – your guide to learning and mastering QuokkaSim, the Rust-based discrete-event simulation framework.

> As of the **28th June 2025**, QuokkaSim is at version `0.2.2` and is under **very active** development. JJ is currently looking for feedback on:
> - **Your experiencing learning QuokkaSim**. Is anything more difficult than it should be? I'd love to hear about it!
> - **Mapping your use cases to QuokkaSim**. Unsure how your particular use case can be modelled with QuokkaSim? I'd love to hear about it and discuss it with you!
> - **Anything in general, really**. QuokkaSim is still in early days - so any and all feedback is appreciated!

---

## What is QuokkaSim?

QuokkaSim is a high-performance, event-driven simulation framework written in Rust, on top of the [NeXosim](https://github.com/asynchronics/nexosim) simulation engine.

QuokkaSim is also:
- Open source
- Permissively licensed (via the MIT License)
- Memory-safe
- Highly performant, via the NeXosim crate
- Accessible to new simulation modellers and new Rust developers

--- 

## Who should read this book?

This book is written primarily for new and experienced simulation modellers, who are new to the Rust programming language and NeXosim simulation engine. The aim of this book (and QuokkaSim more generally) is to provide a structured way to learn QuokkaSim with as minimal friction as possible with the Rust compiler.

For new simulation modellers:
- [Chapter 1: Getting Started](chapter_1.md) is a Quick Start Guide to running and modifying your first QuokkaSim model.
- [Chapter 2: A Conceptual Overview](chapter_2.md) explores the specific concepts used by QuokkaSim, with the aim of new users being able to map real-world scenarios into QuokkaSim concepts, even before touching the code.
- [Chapter 3: Building Your Simulation Model](chapter_3.md) dives into the code, guiding you through how to navigate parsing through an example simulation, and building your own.
- [Chapter 4: Examples](examples.md) summarises a number of example QuokkaSim models, which can be referenced for how to implement different types of custom logic and features.

---

## FAQ

**Why does QuokkaSim use Rust?**

- Incredible performance without the need to write plain C code
    - Rust's performance is comparable to that of C in many benchmarks of the [CLBG](https://benchmarksgame-team.pages.debian.net/benchmarksgame/index.html)
    - Rust performs ~2.5× faster than Java, at ~1/6× the memory consumption ([1](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/nbody.html), [2](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/spectralnorm.html), [3](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/spectralnorm.html))
    - Rust performs ~100× faster than Python, at ~1/8× the memory consumption ([1](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/nbody.html), [2](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/spectralnorm.html), [3](https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/spectralnorm.html))

- Building with Rust means we can make QuokkaSim free, open source and permissively licensed, to [democratise](https://en.wikipedia.org/wiki/Democratization_of_technology) simulation software - making world-class simulation modelling accessible beyond those with licenses to proprietary software.

- Rust's strict compiler ensures all possible cases are explicitly handled and enforces the principle of making impossible states unrepresentable. This aligns perfectly with simulation modeling, where defining clear rules and handling unforeseen edge cases is crucial for reliable extrapolation beyond empirical data and known behaviours.

- Rust is the world's [most admired programming language](https://survey.stackoverflow.co/2024/technology#admired-and-desired-language-desire-admire), and has [maintained this status for the last 8 years](https://github.blog/developer-skills/programming-languages-and-frameworks/why-rust-is-the-most-admired-language-among-developers/)

**Why choose QuokkaSim over SimPy?**

Two main points:
- **Performance** - plain and simple. Faster performance and a smaller memory footprint means not only faster model development and scenario runs, but also unlocks many more opportunities for use e.g. integrations into ML models, embedment in web applications etc.
- **Maintainability** especially for larger simulation models. Different from SimPy, **QuokkaSim** provides a comprehensive framework of components that can be used as-is, or extended by defining custom components, and also benefits from the strictness of Rust's compiler and borrow checker, to provide modellers less time debugging 'plumbing' issues, and more time on business logic and features.

**Yes, this FAQ is bare. Please let JJ know if you have any questions - I'd like to add more to this section!**