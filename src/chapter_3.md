# Chapter 3: A Basic Simulation Model

Now that we've understood the core concepts used by QuokkaSim, let's look at how we assemble a simple simulation model from scratch.

---

## 3.1. Choose Your Data Structures

QuokkaSim supports the following Resource types out-of-the-box:

- **Vectors (f64)**
  - Use when your data can be represented as simple numeric quantities.
  - Example: `VectorStock<f64>` tracks quantities like capacity, consumption, or other scalar values.

- **Vectors (Vector3)**
  - Use when your simulation involves multi-dimensional data (for example, spatial coordinates or RGB color channels).
  - Example: `VectorStock<Vector3>` is great for modeling 3D measurements.

- **Sequences (String)**
  - Use for models where data is represented as sequences or lists of identifiers.
  - Example: `SequenceStock<String>` is useful for handling events or discrete item identifiers.

Your simulation can use a combination of these, or you can create your own resources if this suits your use case better (e.g. if you prefer a vector with named entries for readability, instead of referencing components by index).

|| Vectors (f64) | Vectors (Vector3) | Sequences (String) |
|-|-|-|-|
|Resource|`f64`|`Vector3`|`String`|
|Stock|`VectorStock<f64>`|`VectorStock<Vector3>`|`SequenceStock<String>`|
|Process (1-in, 1-out)|`VectorProcess<f64, f64, f64>`|`VectorProcess<Vector3, Vector3, f64>`|`SequenceProcess<Option<String>>`|
|Process (N-in, 1-out)| TBC | TBC | TBC |
|Process (1-in, N-out)| TBC | TBC | TBC |
|Example use case|Water tanks|Ore movements (using a vector to track elemental composition)|Customers at a Cafe|

**Why so many generic type parameters?**

The pre-defined generic structs `VectorProcess`, `SequenceStock` etc. can be used for any resource your simulation requires - as long as your resource satisfies all trait requirements. These requirements are checked at compiled time by the compiler, so if your code compiles, there's a good chance it will work.

Specifically, for `VectorProcess<T, U, V>`:
- `T` is your resource type
- `U` is the parameter type that is sent by the process to any downstream components, and
- `V` is the parameter type that is sent to upstream components to request material

Thus `VectorProcess<Vector3, Vector3, f64>` is a 1-in, 1-out process which handles a `T=Vector3`, and is passed downstream as a `U=Vector3`. However, when requesting the upstream Stock for material, this process sends a `V=f64` corresponding to the total quantity of desired material, and the stock responds with a `T=Vector3`.