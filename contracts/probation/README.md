# Probation contract

The Probation contract is a complement to the Enforced Classic Asset Wrapper Interface. It implements a Asset Controller interface with specific rules to apply a probation period during which users will have limits applied to their transactions.

# Getting Started

Follow the steps below to deploy and interact with the Probation contract:

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
