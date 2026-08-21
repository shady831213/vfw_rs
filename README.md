# vfw_rs

`vfw_rs` is a Rust-based firmware runtime and infrastructure stack for hardware verification.

It is designed for firmware-driven verification where test code runs close to the hardware model or RTL, while host-side services provide communication, memory access, file I/O, logging, and higher-level verification integration.

The open-source repository contains the reusable infrastructure layer. Real project deployments can add project-specific HALs, integrations, and verification services on top of it.

## Where it fits

```text
                 Host / Simulator / Verification Environment
                                |
                              vhost
                                |
                        mailbox_rs protocol
                                |
                           vfw_mailbox
                                |
                    +-----------+-----------+
                    |                       |
                vfw_core                 vfw_hal
           firmware runtime          hardware access
                    |                       |
                    +-----------+-----------+
                                |
                       Firmware test code
                                |
                         RTL / HW model
```

The stack is intentionally split into reusable layers rather than tying firmware tests directly to one simulator or one testbench.

## Main components

### `vfw_core`

A `no_std` firmware runtime for verification workloads.

It provides infrastructure including:

- architecture-specific runtime support
- heap and stack management
- trap/runtime initialization
- per-hart / per-CPU state
- multicore task execution
- cross-hart `fork` / `fork_on` / `join`
- IPI-based wakeup and synchronization
- acquire/release ordering around task completion
- lightweight messaging and HSM-style state handling
- platform/runtime hooks

The multicore support is intended for firmware tests that need to coordinate work across hardware threads rather than treating every test as a single-core program.

### `vfw_mailbox`

Target-side mailbox integration built on [`mailbox_rs`](https://github.com/shady831213/mailbox_rs).

It provides a `no_std` bridge from firmware code to host-side services, including:

- per-hart mailbox channels
- formatted printing
- generic RPC calls
- test exit/status reporting
- memory operations
- host-backed file access
- configurable pointer width and cache-line layout

This lets firmware tests request services from the host without embedding simulator-specific code into the test itself.

### `vfw_hal`

Reusable HAL pieces for verification firmware. Current modules include clock, DDR, delay, generic I/O, and UART support, built around `embedded-hal` where appropriate.

### `vfw_primitives`

Low-level common primitives shared across the runtime layers.

### `vfw_utils`

Additional reusable utilities for firmware-side test code.

## Multicore verification model

One of the core goals of `vfw_rs` is to make multicore firmware tests straightforward.

A task carries an entry point, arguments, and a `TaskId` containing both hart and task identity. Work can be dispatched to another hart, the target hart is notified through an IPI, and the issuing hart can later `join` the task. The implementation uses explicit memory ordering around completion and wakeup so the synchronization semantics are visible in the runtime rather than hidden in test-specific code.

Conceptually:

```text
hart 0                              hart N
  |                                  |
  |  fork_on(N, task)                |
  |--------------------------------->|
  |                              receive IPI
  |                              run task
  |                                  |
  |<---------------------------------|
  |            completion IPI        |
  |  join(task)                      |
  |                                  |
```

## Mailbox-based host services

`vfw_mailbox` uses `mailbox_rs` as the communication protocol between target firmware and the host environment.

The same mailbox abstraction can be backed by different host integrations. In the broader stack, [`vhost`](https://github.com/shady831213/vhost) connects those services to SystemVerilog DPI/UVM, shared-memory models, optional Python callbacks, and other host-side infrastructure.

This separation is useful for keeping verification firmware portable across environments:

```text
firmware test
    |
 vfw_rs
    |
mailbox protocol
    |
 host service layer
    |
RTL simulator / UVM / model / Python
```

## Public talk

A longer walkthrough of the motivation and the broader stack is available here:

**[Rust For IC design & Verification: vfw, vhost, terminus](https://www.bilibili.com/video/BV1qPe3ezE94/)**

The talk discusses using Rust in IC design and verification flows and shows how `vfw`, `vhost`, and `terminus` fit together.

## Workspace layout

```text
vfw_rs/
|-- vfw_core/        # no_std firmware runtime and multicore execution
|-- vfw_hal/         # reusable hardware abstraction layer
|-- vfw_mailbox/     # target-side mailbox and host-service bridge
|-- vfw_primitives/  # low-level shared primitives
|-- vfw_utils/       # firmware-side utilities
`-- Cargo.toml        # workspace definition
```

## Toolchain

The workspace uses Rust 2021 and currently depends on nightly-only language features in parts of the runtime. Individual crates declare Rust 1.80 as their minimum Rust version where applicable.

For development, use a nightly toolchain when building the complete firmware stack.

## Related projects

- [`mailbox_rs`](https://github.com/shady831213/mailbox_rs) — shared mailbox protocol, queue and RPC infrastructure for host and `no_std` targets
- [`vhost`](https://github.com/shady831213/vhost) — host-side verification integration for mailbox, SystemVerilog DPI/UVM, memory models, Python callbacks, and sockets
- [`terminus`](https://github.com/shady831213/terminus) — RISC-V instruction-set and system simulator
- [`terminus_cosim`](https://github.com/shady831213/terminus_cosim) — ISA/RTL co-simulation environment using the same broader verification infrastructure
