# Merkle Tree Rust (Airdrop-Style Demo)

A simple Rust project that demonstrates how to:

- Build a Merkle tree from transaction claims (`address`, `amount`)
- Compute a Merkle root
- Generate a Merkle proof for one claim
- Verify that proof against the root

This is useful for understanding whitelist/airdrop proof mechanics used in web3 systems.

## Tech Stack

- Rust
- `sha2` crate (`Sha256`)

## How It Works

### 1) Leaf Hashing

Each leaf is built from a claim in this format:

`address:amount`

Then hashed with SHA-256:

`leaf_hash = SHA256(address:amount)`

### 2) Parent Hashing

Nodes are combined in pairs:

`parent = SHA256(left_child || right_child)`

If a level has an odd number of nodes, the last node is duplicated.

### 3) Merkle Proof

A proof is a list of sibling hashes from leaf level up to the root.
Each step also stores whether the sibling is on the **left** or **right**, so verification order is deterministic.

### 4) Verification

Given a claim leaf hash, the proof, and the root:

- Recompute upward using sibling position at each step
- Compare the final computed hash to the Merkle root
- If equal, the proof is valid

## Run

From project root:

```bash
cargo run
```

You should see:

- All input transactions
- Their leaf hashes
- The final Merkle root
- A generated proof for one selected index
- `Is the proof valid? true`

## Project Structure

- `src/main.rs` — full demo implementation (hashing, root, proof generation, verification)
- `Cargo.toml` — project metadata and dependencies

## Notes

- This project is intentionally minimal and educational.
- For production airdrops, keep claim encoding strictly consistent across backend and smart contract verification logic.
- Consider domain separation (e.g. prefix bytes) and test vectors for production-grade systems.
