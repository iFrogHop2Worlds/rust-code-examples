use sha2::{Sha256, Digest};
use hex;

#[derive(Debug)]
struct MerkleTree {
    root: String,
    leaves: Vec<String>,
}

impl MerkleTree {
    fn new(data: &[String]) -> Self {
        let mut leaves: Vec<String> = data
            .iter()
            .map(|x| hash_data(x))
            .collect();

        // If odd number of leaves, duplicate the last one
        if leaves.len() % 2 == 1 {
            leaves.push(leaves.last().unwrap().clone());
        }

        let root = Self::build_tree(&leaves);

        MerkleTree {
            root,
            leaves,
        }
    }

    fn build_tree(leaves: &[String]) -> String {
        if leaves.len() == 1 {
            return leaves[0].clone();
        }

        let mut next_level = Vec::new();

        for chunk in leaves.chunks(2) {
            let combined = format!("{}{}", chunk[0], chunk[1]);
            let parent_hash = hash_data(&combined);
            next_level.push(parent_hash);
        }

        Self::build_tree(&next_level)
    }

    fn get_root(&self) -> &str {
        &self.root
    }
}

fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

fn main() {
    let transactions = vec![
        "Alice->Bob:50".to_string(),
        "Bob->Charlie:30".to_string(),
        "Charlie->David:20".to_string(),
        "David->Alice:10".to_string(),
    ];

    println!("Original Transactions:");
    for tx in &transactions {
        println!("📝 {}", tx);
    }

    let merkle_tree = MerkleTree::new(&transactions);

    println!("\nLeaf Hashes (Transaction Hashes):");
    for (i, leaf) in merkle_tree.leaves.iter().enumerate() {
        println!("🍃 Leaf {}: {}", i + 1, leaf);
    }

    println!("\nMerkle Root:");
    println!("🌳 {}", merkle_tree.get_root());

    let verify_tx = "Alice->Bob:50";
    let verify_hash = hash_data(verify_tx);
    println!("\nVerifying transaction: '{}'", verify_tx);
    println!("Transaction hash: {}", verify_hash);
    println!("Is hash present in leaves: {}", merkle_tree.leaves.contains(&verify_hash));
}