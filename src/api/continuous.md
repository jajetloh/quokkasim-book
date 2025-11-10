# Continuous

**Continuous** refers to entities which deal with **resources** that can take continuous values, such as material mass, liquid volume, energy and so on.

**trait ContProcessCore**
```rust
pub trait ContProcessCore<
    ResourceType: ContResource + 'static,
    LogRecordType: Clone + Send + 'static,
>: Model
{
    fn element_name(&self) -> &str;
    fn element_code(&self) -> &str;
    fn element_type(&self) -> &str;
    fn get_next_event_id(&mut self) -> EventId;
    fn scheduled_event(&mut self) -> &mut Option<(MonotonicTime, ActionKey)>;
    fn previous_check_time(&mut self) -> &mut MonotonicTime;
    fn time_to_next_process_event(&mut self) -> &mut Option<Duration>;

    fn update_state(
        &mut self,
        source_event_id: EventId,
        cx: &mut Context<Self>,
    ) -> impl Future<Output = ()> + Send;
}
```

**trait ContProcessUpdateSinceLast**
```rust
pub trait ContProcessUpdateSinceLast<
    ResourceType: ContResource + 'static,
    LogRecordType: Clone + Send + 'static,
>: ContProcessCore<ResourceType, LogRecordType>
{
    // Resolves the state of this process from the previous update time to now.
    // Handles some edge case handling and logging as well.
    fn update_state_since_last_update(
        &mut self,
        source_event_id: &mut EventId,
        cx: &mut Context<Self>,
    ) -> impl Future<Output = ()> {
        async move {
            ...
            self.update_process_state_since_prev_event(source_event_id, cx, duration_since_prev).await;
        }
    }

    // Concrete function to update the internal process state from the previous time to now
    fn update_process_state_since_prev_event(
        &mut self, source_event_id: &mut EventId,
        cx: &mut Context<Self>,
        duration_since_prev: Duration
    ) -> impl Future<Output = ()>;

    fn log_type_process_success(&mut self, source_event_id: &mut EventId, quantity: f64, resource: ResourceType, cx: &mut Context<Self>) -> impl Future<Output = EventId>;
}

```



**trait ContProcessUpdateDecisionLogic**
```rust
pub trait ContProcessUpdateDecisionLogic<
    ResourceType: ContResource + 'static,
    LogRecordType: Clone + Send + 'static,  
>: ContProcessCore<ResourceType, LogRecordType>
```

**trait ContProcessUpdateForNextEvent**
```rust
pub trait ContProcessUpdateForNextEvent<
    ResourceType: ContResource + 'static,
    LogRecordType: Clone + Send + 'static,
>: ContProcessCore<ResourceType, LogRecordType>
```

**DefaultContProcess**
```
pub struct DefaultContProcess<
    ResourceType: Clone + Send + Debug + 'static,
    ProcessLog: Clone + Send + Debug + 'static,
> { ... }
```

