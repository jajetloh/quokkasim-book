# Chapter 3: A Basic Simulation Model

Now that we've understood the core concepts used by QuokkaSim, let's look at how we assemble a simple simulation model from scratch.

---

## 3.1. Choose Your Data Structures

QuokkaSim supports the following Resource types out-of-the-box:

- **Vectors (f64)**
  - Use when your quantity of interest is a single number that can take a continuous range of values.
  - e.g. For modelling power consumption, your `f64` can represent Energy in Joules
  - e.g. For modelling material flow, your `f64` can represent payload mass in kg
  - `VectorProcess<f64>`, `VectorStock<f64>` and other similar components can be used out-of-the-box

- **Vectors (Vector3)**
  - Use when your quantity of interest has up to three independent components of interest
  - e.g. For modelling nutrient consumption in crops, your `Vector3` can represent masses of Nitrogen, Phsophorous and Potassium elements
  - e.g. For modelling an iron ore mine, your `Vector3` can represent Iron, Silica and other elements by mass
  - `VectorProcess<Vector3>`, `VectorStock<Vector3>` and other similar components can be used out-of-the-box

- **Sequences (String)**
  - Use when your quantity of interest experiencing queuing or other behaviours dependent on other relative ordering
  - e.g. For modelling a queue of vehicles, your `String` can be a unique identifier for a car

Your simulation can use a combination of these, or you can create your own resources if this suits your use case better (e.g. if you prefer a vector with named entries for readability, instead of referencing components by index).

|| Vectors (f64) | Vectors (Vector3) | Sequences (String) |
|-|-|-|-|
|Resource|`f64`|`Vector3`|`String`|
|Stock|`VectorStock<f64>`|`VectorStock<Vector3>`|`SequenceStock<String>`|
|Process (1-in, 1-out)|`VectorProcess<f64, f64, f64>`|`VectorProcess<Vector3, Vector3, f64>`|`SequenceProcess<Option<String>>`|
|Process (N-in, 1-out)|`VectorCombiner<f64, f64, f64, const M: usize>` | `VectorCombiner<Vector3, Vector3, f64, const M: usize>`| TBC |
|Process (1-in, N-out)|`VectorSplitter<f64, f64, f64, const N: usize>`|`VectorSplitter<Vector3, Vector3, f64, const N: usize>`| TBC |
|Process (0-in, 1-out)|`VectorSource<...>`|`VectorSource<...>`|TBC|
|Process (1-in, 0-out)|`VectorSink<...>`|`VectorSink<...>`|TBC|
|Example use case|Water tanks|Ore movements (using a vector to track elemental composition)|Customers at a Cafe|

**Why so many generic type parameters?**

The pre-defined generic structs `VectorProcess`, `SequenceStock` etc. can be used for any resource your simulation requires - as long as your resource satisfies all trait requirements. These requirements are checked at compiled time by the compiler, so if your code compiles, there's a good chance it will work.

Specifically, for `VectorProcess<T, U, V>`:
- `T` is your resource type
- `U` is the parameter type that is sent by the process to any downstream components, and
- `V` is the parameter type that is sent to upstream components to request material

Thus `VectorProcess<Vector3, Vector3, f64>` is a 1-in, 1-out process which handles a `T=Vector3`, and is passed downstream as a `U=Vector3`. However, when requesting the upstream Stock for material, this process sends a `V=f64` corresponding to the total quantity of desired material, and the stock responds with a `T=Vector3`.

## 3.2. The Core Enums

At the beginning of all the example simulations is a section that looks like this:

```rust
define_model_enums! {
    pub enum ComponentModel {}
    pub enum ComponentLogger {}
    pub enum ComponentInit {}
}

impl CustomComponentConnection for ComponentModel {
    fn connect_components(a: &mut Self, b: &mut Self, n: Option<usize>) -> Result<(), Box<dyn Error>> {
        match (a, b) {
            (a, b) => Err(format!("No component connection defined from {} to {} (n={:?})", a, b, n).into()),
        }
    }
}

impl CustomLoggerConnection for ComponentLogger { 
    type ComponentType = ComponentModel;
    fn connect_logger(a: &mut Self, b: &mut Self::ComponentType, n: Option<usize>) -> Result<(), Box<dyn Error>> {
        match (a, b, n) {
            (a, b, _) => Err(format!("No logger connection defined from {} to {} (n={:?})", a, b, n).into()),
        }
    }
}

impl CustomInit for ComponentInit {
    fn initialise(&mut self, simu: &mut Simulation) -> Result<(), ExecutionError> {
        let notif_meta = NotificationMetadata {
            time: simu.time(),
            element_from: "Init".into(),
            message: "Start".into(),
        };
        match self {
            _ => {
                Err(ExecutionError::BadQuery)
            }
        }
    }
}
```

Let's break down what's going on here

``define_model_enums!`` is a macro provided by QuokkaSim, which is used to help define a number of key Enums that are required. In our examples for clarity they are all named `ComponentModel`, `ComponentLogger` and `ComponentInit` respectively. You are able to choose your own names for each in the body of the macro call, but note the order that they are specified determines the role that each enum plays.

- `ComponentModel` is an Enum whose variants are all possible Stocks and Processes in our model. The `define_model_enums!` macro populates this enum with all of QuokkaSim's pre-built components (e.g. `VectorProcessF64(f64)`, `SequenceStockString(SequenceStock<String>)`).

  If we have our own custom-defined components, we can add them to this enum to register them as a usable component.

- `ComponentLogger` is an Enum whose variants are all possible objects that are able to collect logs from our components, and export these logs.

  If we have our own custom-defined loggers, we can add them to this enum to register them as a usable logger.

- `ComponentInit` is an Enum whose variants are all possible ways that objects should be initialised after model creation, but before the first event.

  If we have our own custom initialisations (for our own components, or custom initialisations for existing components), we can add them to this enum to register them as a usable initialisation.

In addition to these enums, a number of special function macros are also defined in `define_model_enums`.

- `connect_components!` is used to define a connection between two components - typically a Stock and Process, or Process to Stock. In the case where a Process has multiple upstream or downstream stocks, an additional parameter is required to determine the specific upstream or downstream stock that is meant.

- `connect_logger!` is used to define a connection between a component and a logger.

- `register_component!` is used to add the constructed component to the model execution engine.

---

After `define_model_enums!` are a number of trait implementations. These trait implementations are how we define the details of what any sort of 'connection' means. Note that the methods implemented in t hese traits have the same name as the function macros defined in `define_model_enums`. This is because each of our functions here are called within the respective function macros if a pre-defined implementation does not exist for the enum variants passed into the function.