# Classic Asset Wrapper

The Classic Asset Wrapper is an approach that aims towards extending classic assets through soroban smart contracts.

Currently, classic assets can already be wrapped in SAC(Stellar Asset Contracts) which are native default contracts that provide a Soroban interface for those assets. These provide with the same key functionalities and behaviors as you'd expect from these assets but accessible in a standardized way following the standard token interface.

Even though this allows for the asset to be integrated in numerous use cases, there is still room for more flexibility since the SACs are not customizable by design. With this in mind the Classic Asset Wrapper can be used in two main ways:

- Soft Wrapper: Provides with an alternative entry point to interact with the asset. This approach is flexible and let's the user and applications decide if they want to leverage the additional programmability provided by the wrapper or not. This can be used in use cases such as:

  - **A rewards program** in which user opt to participate and receive points or cash-back when interacting with the asset.
  - **Use a curated safety whitelist** of applications tha this asset issuer recommends / trusts. Blocking applications that aren't a part of it.

- Hard Wrapper: Replaces the SAC and classic as entry points to interact with this asset, ensuring that all asset holders can only interact with the asset by the implemented rules. This is achieved by a combination of control flags and temporary authorizations managed in contract executions. This is similar to the [SEP08 - Regulated Assets](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0008.md) standard but managed within Soroban instead of an external server. This can be used in use cases such as:
  - **A Regulated Asset** that applies certain limits for accounts in case they haven't been approved by the issuer. This could be tied to a KYC process to ensure that accounts tha't haven't been KYC'd can't transact too often or specific amounts within certain time windows.
  - **Enforce a whitelist** with only specific dApps being allowed by the asset issuer.
  - **Define different tiers of users** for a product token. Depending on your customer tier you might have certain limits and features locked, up to a point where a premium user being fully approved and able to use the asset freely.

This might initially sound like a pure smart contract asset with just extra steps but in reality it aims at allowing for very flexiblae hybrid assets. This ensures that Soroban's programmability can be leveraged wherever necessary to bring flexibility and additional security as well as classic and all of its key strenghts when the objective is to perform fast, cheap and easy-to-integrate transactions.

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

# Classic Wrapper Contract

The classic wrapper contract can be found under `contracts/classic-asset-wrapper-interface`. This contract can be initialized with a custom controller contract to enforce personalized behavior for this classic asset.

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
