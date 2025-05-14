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