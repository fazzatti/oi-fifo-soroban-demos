# Regulated Asset Scripts

This guide provides details on how to use the scripts under the `./scripts` folder to interact with the Regulated Asset and the Asset Controller contracts.

<div align="center">
    <a href="https://youtu.be/vN9thvfEdb0?si=ECqraRSYdcSQJDXi">
        <img src="../../../assets/regulated-asset/thumb-scripts.png" width="450" alt="Check out the Scripts Video">
    </a>
</div>

## Deploy on Futurenet / Testnet

1. Access the folder `/scripts`. There you'll find a collection of helpers scripts to deploy and test this contract.
2. Set the parameters under `config.sh`. By default, they come set for testnet with dummy accounts and assets.
   If this is your first time deploying on testnet, refer to the [Deploy on Testnet](https://soroban.stellar.org/docs/getting-started/deploy-to-testnet) article as you might need to configure your CLI with the testnet parameters.
3. Run `init.sh all` to deploy and initialize both contracts according to the defined parameters.

For further details on how to configure and use the helper scripts, refer to [./scripts/README](scripts/README.md).

## Usage

One can use the `run.sh` helper script to execute the different contract functions. Here below is a list of the supported interactions with the contracts.
For the full list of commands run the following:

```bash
./run.sh h
```

### Retrieving Data with GET

```bash
./run.sh get <CONTRACT> <COMMAND>
```

### Contracts and Commands:

1. **Asset Controller Contract (`ac`)**:

   - `admin`: Retrieve admin information.
   - `asset`: Fetch the asset.
   - `inflow`: Get inflow limit.
   - `outflow`: Get outflow limit.
   - `probation`: Find out probation period.
   - `probation <user>`: Obtain the probation period of a specific user.
   - `quota`: Get the quota time limit.
   - `quota amount <user>`: Determine the quota amount for a user.
   - `quota release <user>`: Find out the quota release time for a user.

2. **Regulated Asset Contract (`ra`)**:
   - `name`: Asset's name.
   - `symbol`: Asset's symbol.
   - `decimals`: Number of decimals.
   - `authorized <USER>`: Check if a user is authorized.
   - `balance <USER>`: Get balance of a user.
   - `spendable <USER>`: Know spendable balance of a user.
   - `allowance <FROM> <SPENDER>`: Get the allowance from one user to another.

### Actors and Commands:

1. **Admin (`admin`)**:

   - `mint <USER> <AMOUNT>`: Mint a specific amount to a user's account.
   - `authorize <USER>`: Authorize or unfreeze a user account.
   - `unauthorize <USER>`: Freeze or revoke authorization from a user account.

2. **User Actions**:
   - `<USER> transfer <USER> <AMOUNT>`: User transfers a specific amount to another user.

The available users for interaction are `a`, `b`, and `c`.
