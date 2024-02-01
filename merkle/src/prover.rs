use crate::util::{
    encode_hash, hash_internal, hash_leaf, write_merkle_proof, Hash32Bytes, MerkleProof,
};

fn gen_leaves_for_merkle_tree(num_leaves: usize) -> Vec<String> {
    let leaves: Vec<String> = (0..num_leaves)
        .map(|i| format!("data item {}", i))
        .collect();

    println!("\nI generated #{} leaves for a Merkle tree.", num_leaves);

    leaves
}

fn hexs(v: Hash32Bytes) -> String {
    let h = hex::encode(v);
    return h.chars().take(4).collect();
}

pub fn gen_merkle_proof(leaves: Vec<String>, leaf_pos: usize) -> Vec<Hash32Bytes> {
    let height = (leaves.len() as f64).log2().ceil() as u32;
    let padlen = (2u32.pow(height)) as usize - leaves.len();

    // hash all the leaves
    let mut state: Vec<Hash32Bytes> = leaves.into_iter().map(hash_leaf).collect();

    // Pad the list of hashed leaves to a power of two
    let zeros = [0u8; 32];
    for _ in 0..padlen {
        state.push(zeros);
    }

    for (index, value) in state.iter().enumerate() {
        println!("{}: {}", index, hexs(*value));
    }

    // initialize a vector that will contain the hashes in the proof
    let mut hashes: Vec<Hash32Bytes> = vec![];

    let mut level_pos = leaf_pos;
    for _level in 0..height {
        // Calculate the position of the sibling node at the current level
        let sibling_pos = if level_pos % 2 == 0 {
            level_pos + 1
        } else {
            level_pos - 1
        };

        // Add the hash of the sibling node to the proof
        hashes.push(state[sibling_pos]);

        // Move to the next level by dividing the position by 2
        level_pos /= 2;

        // Update the state to upper level
        let mut next_state = Vec::new();
        for chunk in state.chunks(2) {
            next_state.push(hash_internal(chunk[0], chunk[1]));
        }
        state = next_state;
    }
    // Compute the Merkle root if we are at the top level
    let merkle_root = state[0];
    // Print the Merkle root
    println!("Merkle Root: {}", encode_hash(merkle_root));

    // Returns list of hashes that make up the Merkle Proof
    hashes
}

pub fn run(leaf_position: usize, num_leaves: usize) {
    let file_name = format!("proof_gen_{}_{}.yaml", num_leaves, leaf_position);

    let leaves = gen_leaves_for_merkle_tree(num_leaves);
    assert!(leaf_position < leaves.len());
    let leaf_value = leaves[leaf_position].clone();
    let hashes = gen_merkle_proof(leaves, leaf_position);

    let mut proof_hash_values_base64: Vec<String> = Vec::new();

    for hash in hashes {
        proof_hash_values_base64.push(encode_hash(hash))
    }

    let proof = MerkleProof {
        leaf_position,
        leaf_value,
        proof_hash_values_base64,
        proof_hash_values: None,
    };

    write_merkle_proof(&proof, &file_name)
}
