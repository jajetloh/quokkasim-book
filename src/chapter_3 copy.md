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


---

## 3.2. Define Your Simulation Components

Your simulation model is built from multiple component types. The main building blocks are:

- **Stock Components**
  - Represent reservoirs or sources/sinks of resources.
  - Pre-defined stocks include `VectorStock` and `SequenceStock`.

- **Process Components**
  - Represent transforms that change data over time.
  - Examples include `VectorProcess` and `SequenceProcess`.

- **Loggers**
  - Capture state changes and other events.
  - Each component can have associated loggers like `VectorStockLogger` or `SequenceProcessLogger` that output CSV logs.

> **Tip:** Examine the code in our examples (e.g. in `sequence_example.rs` and `trucking_2.rs`) to see how components and loggers are instantiated and connected.

---

## 3.3. Assemble and Connect the Simulation

### 3.3.1. Model Construction

- **Configuration Parsing**  
  Read your model configuration from a YAML or JSON file. This file includes details on component names, types, capacities, and distribution settings.

- **Component Instantiation**  
  Create instances of different components based on your configuration.
  
  ```rust
  let mut stock1: SequenceStock<String> = SequenceStock::new()
      .with_name("Stock1".into())
      .with_initial_contents((0..10)
      .map(|x| format!("Item_{:0>4}", x))
      .collect());
  // Create process, additional stocks, and loggers similarly...
  ```

### 3.3.2. Component Connection

- **Connecting Models and Loggers**  
  Use provided macros (like `define_model_enums!`) to connect components and loggers according to your simulation logic.

  ```rust
  ComponentModel::connect_components(
      ComponentModel::SequenceStockString(&mut stock1, &mut stock1_addr),
      ComponentModel::SequenceProcessString(&mut process1, &mut process1_addr),
  ).unwrap();
  ```

### 3.3.3. Simulation Execution

- **Initialization and Event Processing**  
  Assemble the simulation using a builder (like `SimInit`), initialize the simulation with a start time, and process events until your simulation end time.

  ```rust
  let sim_builder = SimInit::new()
      .add_model(stock1, stock1_mbox, "Stock1")
      .add_model(process1, process1_mbox, "Process1");
  let mut simu = sim_builder.init(MonotonicTime::EPOCH).unwrap().0;
  simu.process_event(
      SequenceProcess::<Option<String>, (), Option<String>>::update_state,
      NotificationMetadata { 
          time: MonotonicTime::EPOCH, 
          element_from: "Init".into(), 
          message: "Start".into() 
      },
      &process1_addr
  ).unwrap();
  simu.step_until(MonotonicTime::EPOCH + Duration::from_secs(60)).unwrap();
  ```

---

## 3.4. Running and Debugging Your Simulation

- **Logging Output**  
  Use logger components to output simulation events to CSV files for further analysis. Write logs to disk after simulation execution.

  ```rust
  create_dir_all("outputs/sequence_example").unwrap();
  process_logger.write_csv("outputs/sequence_example".into()).unwrap();
  stock_logger.write_csv("outputs/sequence_example".into()).unwrap();
  ```

- **Adjustment and Debugging**  
  Experiment with changes in component parameters (like capacity and distribution values) and use log outputs to fine-tune simulation behavior.

---

## 3.5. Extending Your Basic Model

- **Adding New Components**  
  Explore how to create custom components by extending pre-defined traits.  
- **Custom Connections**  
  Learn how to implement your own connection strategies using the `CustomComponentConnection` and `CustomLoggerConnection` traits.  
- **Parameter Tuning**  
  Adjust simulation parameters such as seed values, capacity limits, and process timings to explore different model behaviors.

---

## 3.6. Summary

In this chapter, you learned how to:
- Choose appropriate data structures based on your simulation needs.
- Define and instantiate simulation components, processes, and loggers.
- Connect components using provided macros and assemble the simulation.
- Run, log, and debug your simulation model.

In the next chapter, we will dive into more advanced simulation techniques and optimization strategies.

---

*Continue to Chapter 4 for advanced configuration, performance tuning, and real-world case studies.*