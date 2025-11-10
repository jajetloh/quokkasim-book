# Chapter 3: The QuokkaSim Conceptual Model

**Process Event Loop**
1. Figure out what's changed since the previous update time, and execute those changes
2. Figure out what to do next
3. Schedule in the event for the next expected event

For default components provided by QuokkaSim, the above life cycle components are represented by the traits:
1. Cont/DiscProcessUpdateSinceLast
2. Cont/DiscProcessUpdateDecisionLogic
3. Cont/DiscProcessUpdateForNextEvent

**Component Interactions**

- Processes are active - they can perform actions on themselves, and trigger actions on adjacent entities
    - Processes can view the state of connected (upstream/downstream) stocks
    - Processes can push resources into, and withdraw resources out of, adjacent stocks
- Stocks are passive - they only perform actions when triggered by other entities
    - By default, stocks have one of the following states: Empty, Normal or Full. 
    - Stocks can notify adjacent processes when they have a meaningful change of state, and trigger ``update_state`` in those processes. This is because for example, a process may be unable to begin because its upstream stock is Empty, and currently has event scheduled. If the stock is no longer Empty, the process must be triggered in order to start processing - the Stock::state_emitter performs this role of notifying adjacent processes.
