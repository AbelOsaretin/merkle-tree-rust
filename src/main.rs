use sha2::{Digest, Sha256};

#[derive(Clone, Copy)]
struct ProofStep {
    sibling: [u8; 32],
    sibling_is_left: bool,
}

fn to_hex(hash: &[u8; 32]) -> String {
    hash.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn hash_leaf(address: &str, amount: u64) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(address.as_bytes());
    hasher.update(b":");
    hasher.update(amount.to_string().as_bytes());
    hasher.finalize().into()
}

fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

fn calculate_merkle_root(leaves: &[[u8; 32]]) -> Option<[u8; 32]> {
    if leaves.is_empty() {
        return None;
    }

    let mut current_level = leaves.to_vec();

    while current_level.len() > 1 {
        let mut next_level = Vec::with_capacity(current_level.len().div_ceil(2));

        for i in (0..current_level.len()).step_by(2) {
            let left = current_level[i];
            let right = if i + 1 < current_level.len() {
                current_level[i + 1]
            } else {
                left
            };
            next_level.push(hash_pair(&left, &right));
        }

        current_level = next_level;
    }

    Some(current_level[0])
}

fn generate_merkle_proof(leaves: &[[u8; 32]], index: usize) -> Option<Vec<ProofStep>> {
    if leaves.is_empty() || index >= leaves.len() {
        return None;
    }

    let mut proof = Vec::new();
    let mut current_level = leaves.to_vec();
    let mut idx = index;

    while current_level.len() > 1 {
        let (sibling_index, sibling_is_left) = if idx % 2 == 0 {
            let sib = if idx + 1 < current_level.len() {
                idx + 1
            } else {
                idx
            };
            (sib, false)
        } else {
            (idx - 1, true)
        };

        proof.push(ProofStep {
            sibling: current_level[sibling_index],
            sibling_is_left,
        });

        let mut next_level = Vec::with_capacity(current_level.len().div_ceil(2));
        for i in (0..current_level.len()).step_by(2) {
            let left = current_level[i];
            let right = if i + 1 < current_level.len() {
                current_level[i + 1]
            } else {
                left
            };
            next_level.push(hash_pair(&left, &right));
        }

        current_level = next_level;
        idx /= 2;
    }

    Some(proof)
}

fn verify_merkle_proof(leaf_hash: [u8; 32], proof: &[ProofStep], root: [u8; 32]) -> bool {
    let mut computed_hash = leaf_hash;

    for step in proof {
        computed_hash = if step.sibling_is_left {
            hash_pair(&step.sibling, &computed_hash)
        } else {
            hash_pair(&computed_hash, &step.sibling)
        };
    }

    computed_hash == root
}

fn main() {
    let transactions = vec![
        ("0x5C88C720556f41B96885CfCa84458a3492b4839d", 80),
        ("0x1234567890abcdef1234567890abcdef12345678", 100),
        ("0x5B38Da6a701c568545dCfcB03FcB875f56beddC4", 101),
        ("0x4B20993Bc481177ec7E8f571ceCaE8A9e22C02db", 99),
    ];

    let leaf_nodes: Vec<[u8; 32]> = transactions
        .iter()
        .map(|(address, amount)| hash_leaf(address, *amount))
        .collect();
    let merkle_root = calculate_merkle_root(&leaf_nodes).expect("transactions must not be empty");

    println!("Transactions: \n");
    for (addr, amount) in &transactions {
        println!("{}: {}", addr, amount);
        let leaf_hash = hash_leaf(addr, *amount);
        println!("Leaf Hash (address:amount): {}", to_hex(&leaf_hash));
        println!("-----------------------------\n");
    }

    println!("Merkle Root: {} \n", to_hex(&merkle_root));

    let index = 1;
    let (address, amount) = transactions[index];
    let proof = generate_merkle_proof(&leaf_nodes, index).expect("index must be valid");

    println!("Merkle Proof for {}:{}", address, amount);
    for (step_index, step) in proof.iter().enumerate() {
        let side = if step.sibling_is_left {
            "left"
        } else {
            "right"
        };
        println!(
            "  Step {} -> sibling ({}) : {}",
            step_index,
            side,
            to_hex(&step.sibling)
        );
    }
    println!();

    let leaf_hash = hash_leaf(address, amount);
    let is_valid = verify_merkle_proof(leaf_hash, &proof, merkle_root);
    println!("Is the proof valid? {}", is_valid);
}
