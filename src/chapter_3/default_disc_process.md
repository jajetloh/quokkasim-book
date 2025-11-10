# 3.1. Example: DefaultDiscProcess

`DefaultDiscProcess` is a process that withdraws a discrete resource from an upstream stock, holds on to it for a while, then pushes the resource into a downstream stock.

As `DefaultDiscProcess` is a process, it can be polled (from another component, or from its past self) to update its state at some point in time via the `DefaultDiscProcess::update_state` method. Let's take a look at what this method does.

## 3.1.1. Signature

```rust
pub struct DefaultDiscProcess<
    ItemType,
    ProcessLog,
> where
    ItemType: Clone + Debug + Serialize + Send + 'static,
    ProcessLog: Clone + Debug + Serialize + Send + 'static,
{
    ...
}
```

`DefaultDiscProcess<ItemType, ProcessLog>` takes two generic type arguments:
- `ItemType` is the type of the discrete resource, that this process deals with. This can be any type that implements `Clone`, `Debug`, `Serialize` and `Send`, and `'static` labels `ItemType` as having the static lifetime, meaning that it is possible for instances of `ItemType` to exist for the entirety of the program.
- `ProcessLog` is the type of log records that are submitted to the component's built-in log queue. `ProcessLog` has the same requirements in terms of traits, but a lot of the out-of-the-box functionality is built around using `ProcessLog = DiscProcessLog<ItemType>`

Some examples:
- `DefaultDiscProcess<String, DiscProcessLog<String>>` receives a number of `String` resources from an upstream Stock, and sends a number of `String` resources to a downstream Stock.
- `DefaultDiscProcess<MyCustomResource, DiscProcessLog<MyCustomResource>>` performs the same functionality, but instead sends and receives `MyCustomResource` instances. This process still report logs of type `DiscProcessLog<...>`.
- `DefaultDiscProcess<MyCustomResource, MyCustomProcessLog>` uses `MyCustomResource`, but also logs the custom log type `MyCustomProcessLog` to the default log queue.

## 3.1.2. Life Cycle

```
fn update_state(...) -> impl Future<Output=()> {
    async move {
        self.update_state_since_last_update(...).await;
        self.update_state_decision_logic(...).await;
        self.update_state_for_next_event(...).await;
    }
}
```

`update_state` can be broken down into three main parts
- `update_state_since_last_update` updates the internal state of the process, based on the amount of time since the previous update. 

  For example, the process had 60 seconds to completion at the last update, but 50 seconds have passed, then the time to completion is set to 10 seconds. 

  However, let's say there are 10 seconds to completion and it has been 10 or more seconds since the last update - the resource being processed is then sent on to the downstream stock before the remaining logic is executed.

- `update_state_decision_logic` handles decision logic regarding new events.

  In the case of `DefaultDiscProcess`, if no resource is currently being processed, there is available stock upstream, and there is available room downstream, then the process requests stock from upstream to begin processing. The `self.time_to_next_process_event` property is set to the processing duration.

- `update_state_for_next_event` takes the `self.time_to_next_process_event` property, and if a value is provided (is not `None`), an event is scheduled at that duration in the future. Note that `update_state` may be called earlier than this (e.g. if triggered by an adjacent stock changing state).