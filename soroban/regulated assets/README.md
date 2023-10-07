# Regulated Asset dApp demo

## Use Case

Regulated Assets are those that mandate the issuer's (or an authorized third party’s, such as a licensed securities exchange) approval for every transaction. Certain regulations require asset issuers to monitor and approve every transaction involving their assets, ensuring specific constraints are met. The Stellar ecosystem, as defined in [SEP08 - Regulated Assets](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0008.md), offers an approach that capitalizes on Stellar Classic's capabilities. Drawing inspiration from SEP08 standards, this use case enriches a token contract with an auxiliary asset controller contract, allowing the asset issuer to enforce tailored rules programmatically.

<div align="center">
    <a href="https://youtu.be/rec_DnkkJ-A?si=UniAZ9VFVTv6ckGO">
        <img src="../../assets/regulated-asset/thumb-use-case.png" width="450" alt="Check out the Use Case Video explanation">
    </a>
</div>

The hosted demo can be accessed at: [oififo.com/demos/regulated-asset](https://www.oififo.com/demos/regulated-asset)

### The Challenge

SEP08 provides an effective method for creating an Approval Server that evaluates client transactions based on the server's predefined approval criteria. However, these rules operate outside the blockchain. This external operation adds complexity and diminishes transparency for the end user, as the server's code executes off-chain. The advent of Stellar's smart contract platform, Soroban, now offers the opportunity to migrate some of these rules on-chain, ensuring programmable and transparent operations.

Some of the key benefits are:

- **Transparency**: With rules executed on-chain, users can have a clear view of the regulations in place, ensuring a more transparent process than off-chain rules.

- **Flexibility**: The modular nature of Soroban allows asset issuers to implement a range of custom rules, tailored to the specific requirements of the asset.

- **Security**: Leveraging the decentralized and tamper-resistant nature of blockchain, rules and regulations are enforced with greater integrity.

- **Automated Compliance**: By moving rules and regulations on-chain, compliance can be automated, reducing manual intervention and the potential for errors.

### Features

The current version of this use case showcases several core features, highlighting how Soroban's programmable capabilities can incorporate a range of functionalities into the token.

<div align="center">
    <a href="https://youtu.be/T4_Ift9NsQA?si=CLeELlyYhAHw0yBg">
        <img src="../../assets/regulated-asset/thumb-code-overview.png" width="450" alt="Check out the code overview video">
    </a>
</div>

- **Probation Period**: On a user's initial interaction with the regulated asset, either as a sender or a receiver, a probation period begins. This period lasts for a default duration set by the asset issuer. Throughout this period, the asset controller's rules apply to this account. Once the probation ends, the user can engage with the asset as with any standard Soroban token.

- **Outflow Quota**: During probation, there's a restriction on the amount an account can send. It must remain below the outflow limit determined by the asset's administrator. This quota resets periodically based on the asset's predefined time span. For instance, if the outflow quota is set at '100' and the reset period at '24h', it implies that an account can dispatch up to 100 units of the asset every 24 hours. Every transaction deducts from the quota, and once depleted, the user must wait for quota renewal before initiating further transactions.

- **Inflow Quota**: Analogous to the outflow quota, during probation, there's a cap on the amount an account can receive, defined by the asset administrator. This quota too resets after the asset-specific time period.

  > **Important** Both inflow and outflow quotas are managed separatelly.

- **Affiliation(Upcoming)**: This feature, currently under development, aims to establish specific limits for bi-directional interactions between two accounts.

# Getting Started

Follow the steps below to deploy and interact with the Regulated Asset dApp:

## Install Dependencies

1. `soroban-cli v0.9.4`. See https://soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli

## Compile Contracts

This project is composed of two contracts:

- Regulated Asset: A Soroban token implementation with added capabilities to interact with the asset controller.
- Asset Controller: A Rule-enforcing contract tha defines the constratins to be followed by the asset users.

1. In access the `asset-controller` directory, run the command `soroban contract build`. This will compile the Asset Controller contract and generate the wasm files.
2. In access the `regulated-asset` directory, run the command `soroban contract build`. This will compile the Asset Controller contract and generate the wasm files.

## Deploy on Futurenet / Testnet

1. Access the folder `/scripts`. There you'll find a collection of helpers scripts to deploy and test this contract.
2. Set the parameters under `config.sh`. By default, they come set for testnet with dummy accounts and assets.
   If this is your first time deploying on testnet, refer to the [Deploy on Testnet](https://soroban.stellar.org/docs/getting-started/deploy-to-testnet) article as you might need to configure your CLI with the testnet parameters.
3. Run `init.sh all` to deploy and initialize both contracts according to the defined parameters.

For further details on how to configure and use the helper scripts, refer to [./scripts/README](scripts/README.md).

## Usage

One can use the `run.sh` helper script to execute the different contract functions. Here below is an example of a user interacting with the contracts.
For further details on how to configure and use the helper scripts, refer to [./scripts/README](scripts/README.md).

1. The user a send user b 1000 units of the asset.

   ```bash
   ./run.sh a transfer b 100
   ```

2. User a verifies its quota.

   ```bash
   ./run.sh get ac quota a
   ```

3. Admin mints 200 units and sends to user b.
   ```bash
   ./run.sh admin mint b 200
   ```
