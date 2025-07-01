# Chapter 5: API Reference

## Definitions Boilerplate

```rust
use std::error::Error;
use quokkasim::{define_model_enums, prelude::*};

define_model_enums! {
    pub enum ComponentModel {}
    pub enum ComponentModelAddress {}
    pub enum ComponentLogger {}
    pub enum ScheduledEventConfig {}
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
```

## ComponentModel

``ComponentModel`` is an enum whose variants are the possible Stock and Processes in the simulation. A number of pre-built variants are provided with QuokkaSim, and custom variants can also be added for custom components via the ``define_model_enums!`` macro. Variants are named starting with their **Resource**, followed by the type of component for that resource.

- Vector Resources
    - Resource = `f64`
        - ``F64Stock( VectorStock<f64>, Mailbox<...> )``
        - ``F64Source( VectorSource<f64>, Mailbox<...> )``
        - ``F64Sink( VectorSink<f64>, Mailbox<...> )``
        - ``F64Process( VectorProcess<f64, f64, f64, f64>, Mailbox<...> )``
        - ``F64Combiner[N=1-5]( VectorCombiner<f64, f64, [f64; N], f64, N>, Mailbox<...> )``
        - ``F64Splitter[N=1-5]( VectorSplitter<f64, f64, f64, f64, N>, Mailbox<...> )``
    - Resource = `Vector3`
        - ``Vector3Stock( VectorStock<Vector3>, Mailbox<...> )``
        - ``Vector3Source( VectorSource<Vector3, Vector3>, Mailbox<...> )``
        - ``Vector3Sink( VectorSink<f64, Vector3, Vector3>, Mailbox<...> )``
        - ``Vector3Process( VectorProcess<f64, Vector3, Vector3, Vector3>, Mailbox<...> )``
        - ``Vector3Combiner( VectorCombiner[N=1-5]<f64, Vector3, [Vector3; N], Vector3, N>, Mailbox<...> )``
        - ``Vector3Splitter( VectorSplitter[N=1-5]<f64, Vector3, Vector3, Vector3, N>, Mailbox<...> )``
- Discrete Resources
    - Resource = `String`
        - ``StringStock( DiscreteStock<String>, Mailbox<...> )``
        - ``StringSource( DiscreteSource<String, String, StringItemFactory>, Mailbox<...> )``
        - ``StringSink( DiscreteSink<(), Option<String>, String> )``
        - ``StringProcess( DiscreteProcess<(), Option<String>, String, String>, Mailbox<...> )``
        - ``StringParallelProcess( DiscreteProcess<(), Option<String>, String, String>, Mailbox<...> )``
    - Resource = `F64Container`
        - ``F64ContainerStock( DiscreteStock<F64Container>, Mailbox<...> )``
        - ``F64ContainerSource( DiscreteSource<F64Container, F64Container, StringItemFactory>, Mailbox<...> )``
        - ``F64ContainerSink( DiscreteSink<(), Option<F64Container>, F64Container> )``
        - ``F64ContainerProcess( DiscreteProcess<(), Option<F64Container>, F64Container, F64Container>, Mailbox<...> )``
        - ``F64ContainerParallelProcess( DiscreteProcess<(), Option<F64Container>, F64Container, F64Container>, Mailbox<...> )``
        - ``F64ContainerLoadProcess( ContainerLoadingProcess<F64Container, f64>, Mailbox<...> )``
        - ``F64ContainerUnloadProcess( ContainerLoadingProcess<F64Container, f64>, Mailbox<...> )``
    - Resource = `Vector3Container`
        - ``Vector3ContainerStock( DiscreteStock<Vector3Container>, Mailbox<...> )``
        - ``Vector3ContainerSource( DiscreteSource<Vector3Container, Vector3Container, StringItemFactory>, Mailbox<...> )``
        - ``Vector3ContainerSink( DiscreteSink<(), Option<Vector3Container>, Vector3Container> )``
        - ``Vector3ContainerProcess( DiscreteProcess<(), Option<Vector3Container>, Vector3Container, Vector3Container>, Mailbox<...> )``
        - ``Vector3ContainerParallelProcess( DiscreteProcess<(), Option<Vector3Container>, Vector3Container, Vector3Container>, Mailbox<...> )``
        - ``Vector3ContainerLoadProcess( ContainerLoadingProcess<F64Container, f64>, Mailbox<...> )``
        - ``Vector3ContainerUnloadProcess( ContainerLoadingProcess<F64Container, f64>, Mailbox<...> )``