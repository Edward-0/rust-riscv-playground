# rust-riscv-playground
A simple not yet finished MIDI driver for the HiFive-Revb to try out embedded Rust and RISC-V

## Panic
Rust requires you to implement a panic function if not using the standard library. The not yet fully working panic that i've implemented is meant to
turn on the red LED and hang.

## Alloc
To use Rusts collections API alloc needs to be implemented. I have done this using some basic boilerplate around the C `malloc` and `free` functions.
