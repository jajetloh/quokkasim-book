# The QuokkaSim Book

Welcome to the **QuokkaSim Book** – your guide to learning and mastering QuokkaSim, the Rust-based discrete-event simulation framework.

> As of the **8th June 2025**, QuokkaSim is at version `0.1.0` and is under **very active** development. JJ is currently looking for feedback on:
>
> - Your experiencing learning QuokkaSim, no matter your prior experience
>   - Are the main concepts (Resource, Stocks, Processes, Loggers etc.) intuitive?
    - Is it easy to map real-world scenarios into the above concepts?
>   - Are the tutorials here and examples in the repository easy to follow?
>   - Are you having to fight the Rust compiler? If yes, what kind type of issues are you facing? A quick onboarding to Rust is within the scope of what I would like this guide to cover
>   - Is it easy to know how to implement model features you have in mind? This is also within the scope of what I would like this guide to cover (at least briefly)
>   - How is the model debugging process with logged events? Is the provided logging sufficient to help debug issues, or are you having to dive deeper?
> - Pain points whilst developing
>   - Are compiler and runtime errors informative enough to debug easily?
>   - What is tedious and repetitive? Future releases will have utilities to reduce repetition and boilerplate
> - Bugs and issues
>   - Unsure if you've found a bug? Let me know anyway! Even if it's not a bug, it may be something unintuitive or unclearly documented, which I am interested in improving
>   - Unsure about things in general? Let me know anyway! Your uncertainty may be a great prompt for content to include in the FAQ or other areas of the documentation or book
> - Use cases
>   - Let's chat if you think you have a use case for QuokkaSim, or if you need any help with mapping your use case into QuokkaSim! I want the modelling process to be as easy as possible - and that includes mapping from a real-world scenario, into a runnable model.


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

**Yes, this FAQ is bare. Please let JJ know if you have any questions - I'd like to add more to this section!**