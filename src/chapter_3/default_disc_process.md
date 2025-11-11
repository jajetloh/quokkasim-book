# 3.1. Anatomy of a Process: `DefaultDiscProcess`

Although QuokkaSim components can be used straightaway with minimal configuration, many uses cases will require small adjustments to default logic. More specialised use cases may need entirely custom code that is able to interface with other QuokkaSim components. For any degree of customisation, an understanding of how a Process works under-the-hood is require

Once you understand the inner workings, you can play with and break them as needed.

---

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
    fn update_state(
        &mut self,
        source_event_id: EventId,
        cx: &mut Context<Self>,
    ) -> impl Future<Output = ()> { ... }
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


```rust
fn update_state(
    &mut self,
    source_event_id: EventId,
    cx: &mut Context<Self>,
) -> impl Future<Output=()> {
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

  Taking a closer look at the definition of `update_state_since_last_update`, there is some handling of the some internals like `self.scheduled_event()`, but there is also a method `self.update_process_state_since_prev_event()`.

  ```rust,editable
  fn update_state_since_last_update(
      &mut self,
      source_event_id: &mut EventId,
      cx: &mut Context<Self>,
  ) -> impl Future<Output = ()> {
      async move {
          if let Some((scheduled_time, _)) = self.scheduled_event() {
              if *scheduled_time <= cx.time() {
                  *self.scheduled_event() = None;
              }
          }

          let duration_since_prev = cx.time().duration_since(*self.previous_check_time());

          self.update_process_state_since_prev_event(source_event_id, cx, duration_since_prev).await;
      }
  }
  ```
  - `update_process_state_since_prev_event()` contains the particular logic of how to resolve the process state, provided the duration since the last event. This means in most cases where we wish to write our own logic, it suffices to provide our own version of `update_process_state_since_prev_event`, instead of having to overwrite `update_state_since_last_update`. We will see how to do this later on.

- `update_state_decision_logic` handles decision logic regarding new events.

  In the case of `DefaultDiscProcess`, if no resource is currently being processed, there is available stock upstream, and there is available room downstream, then the process requests stock from upstream to begin processing. The `self.time_to_next_process_event` property is set to the processing duration.

- `update_state_for_next_event` takes the `self.time_to_next_process_event` property, and if a value is provided (is not `None`), an event is scheduled at that duration in the future. Note that `update_state` may be called earlier than this (e.g. if triggered by an adjacent stock changing state).

  Even for most custom components, the default implementation of `update_state_for_next_event` can be used. This method only needs to be overridden in special cases, like if additional logging is required during this step.

### 3.1.3. Customising Processes

Previously we explored what these default functions do:
```
update_state(...)
    update_state_since_last_update(...)
        update_process_state_since_prev_event(...)
    update_state_decision_logic(...)
    update_state_for_next_event(...)
```

This default functionality is provided to `DefaultDiscProcess` by implementing certain traits. Each of the functions is provided by a certain trait, and each can be customised by providing a custom implementation of that function, when implementing that trait.

|Trait|Function/s|
|--|--|
|DiscProcessCore|update_state|
|DiscProcessUpdateSinceLast|update_state_since_last_update<br/>update_process_state_since_prev_event|
|DiscProcessUpdateDecisionLogic|update_state_decision_logic|
|DiscProcessUpdateForNextEvent|update_state_for_next_event|

In the next chapter, we'll explore how to implement custom logic by providing custom implementations for these methods.