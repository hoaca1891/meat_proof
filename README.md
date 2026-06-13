# meat_proof

## Project Title
meat_proof

## Project Description
meat_proof is a Soroban smart contract that brings end-to-end traceability to the meat supply chain. Today, consumers, retailers, and regulators have no cheap, tamper-proof way to verify that the beef, lamb, or poultry they buy actually came from a registered animal, was slaughtered at a certified plant, and went through the processing steps the label claims. meat_proof fixes that by recording every link in the chain on the Stellar blockchain: a rancher registers an animal with an off-chain certification hash, a slaughter plant turns that animal into a numbered batch, processors attach each transformation step (cut, package, refrigerate, ship, ...) to the batch, and a certifier can attach a halal-compliance flag. Any consumer can then verify, with a single contract call, how many traceable steps exist between their meat and the source animal.

## Project Vision
The long-term goal of meat_proof is to make food-fraud and mislabeling economically irrational. By anchoring the chain of custody in a public, append-only ledger, the project aims to give every participant - from smallholder ranchers to halal certification bodies to end consumers - a shared, trustless source of truth about where meat came from and what happened to it. The vision is a global, permissionless traceability layer that any farm, plant, retailer, or regulator can plug into without having to build or run their own private infrastructure, leveraging Stellar's low fees and fast finality to make per-batch on-chain recording viable at commodity scale.

## Key Features
- Animal registration with certification hash: a rancher signs a transaction to register a unique animal tag, breed code, and an off-chain certification hash (organic, grass-fed, free-range, etc.) that downstream auditors can verify.
- Slaughter-to-batch anchoring: a slaughter plant records the slaughter event, binding a freshly minted batch ID to a previously registered animal tag along with the carcass weight and slaughter date.
- Append-only processing steps: processors, packagers, and logistics operators can each append a transformation step (step name and location) to a batch, building a verifiable chain of custody that grows over time.
- Consumer verification in one call: the `verify` view function returns the number of recorded processing steps for a batch, giving consumers a quick, gas-light way to assess traceability depth.
- Halal certification flag: an authorized certifier can set or revoke a halal-compliance flag on a batch, and the `is_halal` view lets retailers and consumers filter or display compliance status without trusting the producer.
- On-chain authorization: every state-changing function requires `require_auth()` from the responsible party (rancher, plant, processor, or certifier), so no single account can rewrite history on behalf of others.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** supply_chain dApp — see `contracts/meat_proof/src/lib.rs` for the full meat_proof business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CB2UNFPEHKRRXSLD2TLPAGXCJFXFONMDVO65J6LQVOVYN63RRIMDN4NN`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/68f884c856a8e838d429d841a09d36af0a7fee3004e8d96838f85288ca78b029`
- **Screenshot of deployed contract on Stellar Expert:**

![screenshot](https://i.ibb.co/7tHF2mcF/image.png)


## Future Scope
- Off-chain document anchoring: integrate IPFS or a similar content-addressed store so that `cert_hash` can resolve to a full certificate PDF and so that each processing step can attach richer evidence (photos, lab reports, temperature logs).
- Multi-signature regulatory oversight: add a role-based access layer so that government food-safety authorities can co-sign recall events, freezing a batch on-chain the moment a contamination is detected.
- QR-code consumer app: build a lightweight web or mobile frontend that scans a QR code on packaged meat, looks up the batch on Stellar, and renders the full farm-to-shelf journey including the halal status.
- Cross-chain and asset hooks: extend the contract to optionally hold a small escrow (USDC on Stellar) that is released to the rancher only after the slaughter event is recorded, turning traceability into a built-in payment rail.
- Privacy-preserving attestations: add zero-knowledge or hash-based attestations so that commercial-sensitive data (exact weight, customer list) can stay private while still producing a publicly verifiable proof of compliance.
- Analytics dashboard: an off-chain indexer that aggregates `verify` and `is_halal` calls into a public dashboard showing average traceability depth, certification coverage, and recall response times per region.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `meat_proof` (supply_chain)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
