# Chapter 2: A Conceptual Overview

In Chapter 1 we ran some code for our first simulation, but now we'll try to cover the core concepts used for modelling in QuokkaSim, before we look again at the code to understand better what's happening.

---

## 2.1. Discrete Event Simulation and the Schedule

Simulations answer "what-if" questions, most commonly about what is the result of a set of initial conditions, rules, and controls.

One approach is to approximate the continuous flow of time, by simulating the system at regular time-steps. This is a good regime for modelling continuous processes, such as:

- Movement of a pendulum under gravity,
- Temperature of a room over a day, or
- Stresses and fatigue in a beam under repeated loads

A time-stepped approach is good in these cases because
1. There are well-defined equations for how quantities of interest change over time
2. The length of the time-step can be adjusted based on how precise results need to be (and in fact, some approaches [use an adaptive, changing time-step](https://en.wikipedia.org/wiki/Adaptive_step_size))

In each of the above situations, quantities of interest change continuously over time, so we need to re-calculate the state of the system often to ensure we're capturing any important changes.

However, many systems don't deal with continuous quantities, or have important behaviour happening continuously over time, for example:

- Traffic flow through a traffic light,
- Preparing drinks at a cafe, or
- Car assembly at a factory

In these situations, it can be very inefficient to update every component of the system at every timestep. A better approach here is to only update the system at the exact timesteps where something will happen! This is the approach taken in Discrete Even Simulation.

In Discrete Event Simulation, an ordered schedule of events is maintained, and time jumps forward to the next event, which in turns can schedule more events.

## 2.2. Resources, Stocks and Processes

TL;DR:
- Resources are the quantities of stuff that we're interested in the movement of
- Stocks are entities which passively hold resources
- Processes are entities which actively move or transform resources, and can hold some resources as part of processing

| Example Use Case | Resource | Stocks | Processes |
|----------|-|-------|---------|
|Water tanks connected by pipes|Litres of water|Tanks, resevoirs, pipes|Valves|
|Trucking of Ore|Tonnes of ore|Stockpiles of ore|Trucks, conveyors, reclaiming equipment|
|Customers in a Cafe|Customers|Queue for ordering, waiting area|Taking orders, serving drinks|

## 2.3. Detailed Material Flow: Customers in a Cafe

**Time = Initial**
<p align="center"><img src="images/barista_1.svg" alt="Barista 1" /><p>
We have a stock representing a queue of customers, which is processed by a barista who takes drink orders, before customers go into another queue to wait for their drink. The starting point of the simulation is shown in the diagram, with 3 people in line, and 1 person waiting for their drink.

Remember that Discrete Event Simulations rely on scheduled events triggering more events. There are no events at the start of the simulation, so we kick-start the simulation at the start by telling our Process to update its own state.

**Time = 0:00**
<p align="center"><img src="images/barista_2.svg" alt="Barista 2" /><p>

The barista can only process a customer if a customer is in the queue, and if there is sufficient room in the waiting area (or perhaps just to not overwhelm the baristas who are making the drinks).

So first, the process retrieves the current state of the upstream stock (Queued Customers) and downstream stock (Waiting for drinks). For now, stocks can have one of three states: Empty, Normal and Full.

Queued customers is Full and Waiting for drinks is Normal. In this case, the barista can take an order, so the process seizes the front-most customer from the queue, and schedules a new event to occur in 30 seconds, when the ordering is complete.

**Time = 0:30**
<p align="center"><img src="images/barista_3.svg" alt="Barista 3" /><p>

30 seconds later, the event triggers *Barista taking orders* is prompted to update itself again. This time, it first moves Customer B moves into the *Waiting for drinks* stock, before checking upstream and downstream states again. This time both states are *Normal*, in which case the process seizes the next customer and repeats.

<p align="center"><img src="images/barista_4.svg" alt="Barista 4" /><p>

**Time = 1:00**

<p align="center"><img src="images/barista_5.svg" alt="Barista 5" /><p>

*Barista taking orders* is again triggered to update its state, and moves C into *Waiting for drinks*. This time the upstream and downstream states are *Normal* and *Full*. Because downstream is full, we do not process the next customer, and instead wait until we are triggered to update again.

**Time = 1:15**
<p align="center"><img src="images/barista_6.svg" alt="Barista 6" /><p>

*Waiting for drinks* has reduced from 3 people to 1 person, meaning its state has changed from *Full* to *Empty*. This change of state triggers any upstream or downstream processes to update, meaning that *Barista taking orders* is triggered to update.

*Barista taking orders* checks both upstream and downstream state again, finds both are *Normal*, so then begins processing D.

## 2.4. A Taxonomy of Processes

In QuokkaSim, Processes are at the heart of all dynamic behaviours. We wish to make our processes as flexible as possible to allow us to test whatever we wish, but also to have as few different types of process, to minimise complexity and the mental burden of reasoning about them. With a small set of well-defined components, modellers who are new to QuokkaSim are able to pick up the framework quickly, and effectively translate real-world scenarios to a model representation.

Note that we are not saying you have to follow this taxonomy or these concepts. You are free to create your own components suited to your use cases. If you have a case that isn't well suited to using our components, we'd like to hear about it! If such use cases are common, we will consider adding them as components in QuokkaSim directly.

| Concept | # Inputs | # Outputs | Conserves resources? |
|-|-|-|-|
| Simple Process | 1 | 1 | ✅ Yes|
| Source | 0 | 1 | ❌ Creates resources |
| Sink | 1 | 0 | ❌ Destroys resources |
| Combiner | N | 1 | ✅ Yes |
| Splitter | 1 | N | ✅ Yes |

### 2.4.1. Resource Conservation

In most systems, there can be a natural sense of conservation, or of balance. For example:

- In a transport simulation, a `Car { car_id: u32, make: String, model: String }` resource should not be created or destroyed in most processes. Cars are a conserved quantity in this context.
- In a manufacturing simulation where we track the parts of a car through the manufacturing process via a vector of masses `ProtoCar { assembled_kg: f64, loose_tyres_kg: f64, loose_engine_kg: f64 }`, the total mass of parts through our system should be conserved, except where new parts are introduced into the system or the assembled car is removed from the system. However, the individual components like `loose_tyres_kg` are not conserved, and can be transformed into `assembled_kg` subject to the total mass being conserved.
- In an ore crushing operation, where a crusher processes large rocks into smaller pieces, we may use a vector of masses `Ore { small: f64, medium: f64, large: f64 }`. The crusher may transform large pieces into small or medium pieces, but the total mass is conserved.

By having most processes conserving resources, the key activities of resource and creation are separated out into their own proceses, which can be monitored with more scrutiny than other processes.


### 2.4.2. Types of Combiners and Splitters

Combiners and Splitters can be further classified by whether or not inputs/outputs are **AND-like (combinations)** or **XOR-like (choices)** - or a combination of both.

What does this mean? Let's think through a couple of examples of combiners first.

<p align="center"><img src="images/combiner_xor.svg" alt="XOR Combiner" /><p>

Let's say you're a delivery driver. At your disposal is a number of cars, vans and trucks. Each distinct set of vehicle is a stock, but you can only choose to take from one of these stocks at this time. This is an **XOR-like** combiner process, as you must choose a car **or** a van **or** a truck - where **or** is an **exclusive or** (XOR).

<p align="center"><img src="images/combiner_and.svg" alt="AND Combiner" /><p>

On the other hand, let's say you are combining the ingredients for a cake - butter, sugar, eggs and flour. In this case, we want a combination of all four in a specific ratio, not an exclusive choice of one of them. In general we want a **combination** of some quantity of butter **and** sugar **and** eggs **and** flour (though in an edge case some quantities may be 0), thus this is a **AND-like** process.

An example of a more general combiner which has both XOR and AND elements is the above cake batter example, but where you have to choose exclusively from different types of sugar like White Sugar, Brown Sugar and Honey.

A similar breakdown can be made of Splitters. XOR-like splitters push material into one stock at a time. AND-like splitters generally push material into all downstream stocks at once. And a more general splitter can push into multiple, but have some exclusivity constraints.