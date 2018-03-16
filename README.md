ai_behavior [![Build Status](https://travis-ci.org/PistonDevelopers/ai_behavior.svg?branch=master)](https://travis-ci.org/PistonDevelopers/ai_behavior)
===========

AI behavior tree

You can serialize the
behavior tree using [Serde](https://crates.io/crates/serde) and
e.g. [Ron](https://crates.io/crates/ron).

### What is an AI behavior tree?

An AI behavior tree is a kind of state machine logic for processes.

Many things that a game logic does, e.g. controlling AI characters,
fits the pattern of AI behavior trees.

An AI behavior tree is a very generic way of organizing interactive logic.
It has built-in semantics for processes that signals `Running`, `Success` or
`Failure`.

For example, if you have a state `A` and a state `B`:

- Move from state `A` to state `B` if `A` succeeds: `Sequence([A, B])`
- Try `A` first and then try `B` if `A` fails: `Select([A, B])`
- Do `B` repeatedly while `A` runs: `While(A, [B])`
- Do `A`, `B` forever: `While(WaitForever, [A, B])`
- Wait for both `A` and `B` to complete: `WhenAll([A, B])`
- Wait for either `A` or `B` to complete: `WhenAny([A, B])`

See the `Behavior` enum for more information.

### Parallel semantics

This library has parallel semantics for AI behavior trees.
It means that multiple processes can happen at the same time
and the logic can be constructed around how these processes runs or terminate.

For example, `While(A, [B])` runs both `A` and `B` at the same time.
If either `A` or `B` fails, then the whole while-behavior fails.

A property of AI behavior trees with parallel semantics is that you can
control termination conditions externally, as opposed to most
programming languages where termination condition is controlled internally:

```text
while A() {
    // This inner loop will never terminate unless `B` fails.
    while true {
      B();  // Runs `B` forever.
    }
}
```

```text
// This will terminate if `A` stops running, which also stops `B`.
WhenAny([A,
  While(WaitForever, [
    B
  ])
])
```

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)
