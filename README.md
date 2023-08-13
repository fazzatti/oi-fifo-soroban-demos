# Oi Fifo - Stellar Soroban Use Cases

A collection of various use cases implemented on Stellar's smart contract platform, Soroban.

<div align="center" style="display: flex; justify-content: space-between; align-items: center;">
  <img src="assets/logo_oififo.png" alt="Oi Fifo Logo" style="width: 50px; align-self: center; margin-right: 50px;" />
  <img src="https://soroban.stellar.org/img/soroban-wordmark-temp.svg" alt="Soroban" style="width: 200px; align-self: center;" />
</div>


## Use Cases

### Regulated Asset ([Access](soroban/regulated%20assets/README.md))
Regulated Assets are those that mandate the issuer's (or an authorized third party’s, such as a licensed securities exchange) approval for every transaction. Certain regulations require asset issuers to monitor and approve every transaction involving their assets, ensuring specific constraints are met. The Stellar ecosystem, as defined in [SEP08 - Regulated Assets](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0008.md), offers an approach that capitalizes on Stellar Classic's capabilities. Drawing inspiration from SEP08 standards, this use case enriches a token contract with an auxiliary asset controller contract, allowing the asset issuer to enforce tailored rules programmatically.