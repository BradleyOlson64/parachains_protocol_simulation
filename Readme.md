# Polkadot Parachains Protocol Simulation

Learn the steps by which Polkadot provides shared security to parachains!

## Table of Contents

### Chapter 1: Happy Path

We establish an understanding of the basic flow which a valid parachain block candidate would pass through to eventually be included in a finalized relay chain block.

- Part 1 - Partial Happy Path - We start with the big pieces, leaving out less well known intermediate steps of the happy path
- Part 2 - Full Happy Path - We start to get a full picture of the Polkadot parachains protocol

### Chapter 2: Unhappy Paths

We establish a very simplified notion of parachain state which is changed by parachain blocks. It is now possible for parachain blocks to include invalid state transitions. So it is up to validators on the relay chain to catch and reject bad parachain block candidates (parablocks) before they can reach finality. We also start to introduce the notion of malicious validators and cover how the parachains protocol deals with some of their malicious actions.

- Part 1 - Introducing Bad Parablocks - Covers how an honest backing set of validators should reject bad parablocks without having to trigger the more resource intensive approvals process.
- Part 2 - Introducing Malicious Backers - 
- Part 3 - Introducing Malicious Approver False Positives - Here, false positive means that an approver disputed a parachain block that they know to be valid. Only a malicious approver would do this.
- Part 4 - Introducing Broken Parablock Liveness - Here we demonstrate how a > 1/3 malicious validator set can stall the availability of valid parablocks or prevent disputes from concluding.
- Part 5 - Introducing Broken Parablock Security - Demonstrate how 

## License

Licensed under the terms of the [GPL-3](https://www.gnu.org/licenses/gpl-3.0.en.html) or later.

## Context

This content was written to accompany module 6 of the Polkadot Blockchain Academy, Buenos Aires 2023.

It strives to be useful stand-alone content as well.