# Classic Asset Wrapper Workflow

This subproject containes some helper scripts and tests to play around the Classic Asset Wrapper use case.

# Getting Started

## Setup

When using the repository for the first time, first access this workflow directory and run the following command to install the dependencies.

```bash
pnpm install
```

Afterwards, ensure you have compiled the contracts by accessing their directories under `contracts/*` and running `stellar contract build`. See the contract's readme for more details.

## Running the project

To faciliate interacting with the project, we made a Makefile with the main commands. These should help you take the first steps but we also encourage you to dig through the code base and change it as part of your learning journey.

To access the commands, run:

```bash
make help
```

or

```bash
make h
```

This command should output the help details of all commands available and their features.

# Use cases

As this repository grows over time, the objective is to share different use cases and approaches on how to implement and use the Classic Asset wrapper. See the following subsections for specifics of these use cases.

## Asset Controller

**Type:** Hard Wrapper
**Directory:** `contracts/asset-controlle`

The Asset Controller use case implements specific rules to an asset. These apply a probation period during which users will have limits enforced upon to their transactions.

## Features

Once initialized, whenever a user transacts with asset wrapper that integrates this controller, they will be affected by its rules.

Once a user first receives tokens through a transfer, they'll enter the _Probation_ period. During this period, they'll have specific quotas applied to how many tokens they can receive(inflow quota) at certain interval as well as how many tokens they can send(outflow quota) at the same intervals.

After the probation period ends or the asset admin approves their trustline, the limits won't be applied anymored.

- **Probation Period**: On a user's initial interaction with the regulated asset, either as a sender or a receiver, a probation period begins. This period lasts for a default duration set by the asset issuer. Throughout this period, the asset controller's rules apply to this account. Once the probation ends, the user can engage with the asset as with any standard Soroban token.

- **Outflow Quota**: During probation, there's a restriction on the amount an account can send. It must remain below the outflow limit determined by the asset's administrator. This quota resets periodically based on the asset's predefined time span. For instance, if the outflow quota is set at '100' and the reset period at '24h', it implies that an account can dispatch up to 100 units of the asset every 24 hours. Every transaction deducts from the quota, and once depleted, the user must wait for quota renewal before initiating further transactions.

- **Inflow Quota**: Analogous to the outflow quota, during probation, there's a cap on the amount an account can receive, defined by the asset administrator. This quota too resets after the asset-specific time period.

  > **Important** Both inflow and outflow quotas are managed separatelly.
