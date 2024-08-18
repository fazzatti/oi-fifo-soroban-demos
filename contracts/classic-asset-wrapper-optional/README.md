# Optional Classic Asset Wrapper contract

The Optional Classic Asset Wrapper contract implements an optional classic wrapper interface to provide an alternative way of interacting with a classic asset, enabling extended custom behavior.

# Getting Started

Follow the steps below to deploy and interact with the Optional Classic Asset Wrapper contract:

## Install Dependencies

1. `stellar-cli v21.3.0`. See https://github.com/stellar/stellar-cli

## Building the contract

Run the command below to compile the contracts in this workspace:

```bash
stellar contract build
```

## Using the contracts

For more detailed steps to interact with this contract, check out the workflow project under `workflows/classic-asset-wrapper`.

## Running Tests

The tests are located under `src/test.rs` and can be triggered with the following command:

```bash
cargo test
```
