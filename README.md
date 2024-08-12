# Oi Fifo - Stellar Soroban Use Cases

<table align="center" border="0">
<tr>
<td><img src="assets/logo_oififo.png" alt="Oi Fifo Logo" width="150"/></td>
<td><img src="https://soroban.stellar.org/img/soroban-wordmark-temp.svg" alt="Soroban" width="250"/></td>
</tr>
</table>

A collection of various use cases implemented on Stellar's smart contract platform, Soroban.

## Use Cases

### Regulated Asset ([Access](contracts/regulated-token/README.md))

Regulated Assets are those that mandate the issuer's (or an authorized third party’s, such as a licensed securities exchange) approval for every transaction. Certain regulations require asset issuers to monitor and approve every transaction involving their assets, ensuring specific constraints are met. The Stellar ecosystem, as defined in [SEP08 - Regulated Assets](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0008.md), offers an approach that capitalizes on Stellar Classic's capabilities. Drawing inspiration from SEP08 standards, this use case enriches a token contract with an auxiliary asset controller contract, allowing the asset issuer to enforce tailored rules programmatically.

<div align="center">
    <a href="[https://youtu.be/rec_DnkkJ-A?si=UniAZ9VFVTv6ckGO](https://youtube.com/playlist?list=PLJo7htkGqBrFIHg6keCRRYjf6AEB1m1wo&si=trRFyeLCELwJeGNA)">
        <img src="assets/regulated-asset/thumb-use-case.png" width="450" alt="Check out the video playlist for this use case">
    </a>
</div>

The hosted demo can be accessed at: [oififo.com/demos/regulated-asset](https://www.oififo.com/demos/regulated-asset)
