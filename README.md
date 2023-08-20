# Oi Fifo - Stellar Soroban Use Cases
<table align="center" border="0">
<tr>
<td><img src="assets/logo_oififo.png" alt="Oi Fifo Logo" width="150"/></td>
<td><img src="https://soroban.stellar.org/img/soroban-wordmark-temp.svg" alt="Soroban" width="250"/></td>
</tr>
</table>

<<<<<<< HEAD
A collection of various use cases implemented on Stellar's smart contract platform, Soroban.

<div style="display: flex; align-items: center; justify-content: space-between;">
  <img src="assets/logo_oififo.png" alt="Oi Fifo Logo" width="100" />
  <img src="https://soroban.stellar.org/img/soroban-wordmark-temp.svg" alt="Soroban" width="250" />
</div>


=======
A collection of use cases implemented on Stellar's smart contract platform, Soroban.
>>>>>>> 41483b3b2ea9b6d5f09b09b8df8ecc7e099b7112

## Use Cases

### Regulated Asset ([Access](soroban/regulated%20assets/README.md))
Regulated Assets are those that mandate the issuer's (or an authorized third party’s, such as a licensed securities exchange) approval for every transaction. Certain regulations require asset issuers to monitor and approve every transaction involving their assets, ensuring specific constraints are met. The Stellar ecosystem, as defined in [SEP08 - Regulated Assets](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0008.md), offers an approach that capitalizes on Stellar Classic's capabilities. Drawing inspiration from SEP08 standards, this use case enriches a token contract with an auxiliary asset controller contract, allowing the asset issuer to enforce tailored rules programmatically.
