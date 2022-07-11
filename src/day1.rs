use sp_core::{hexdisplay::*, *, hashing::sha2_256};

pub fn pair() -> sr25519::Pair {
    let (pair, mnemonic, raw_seed )  = sr25519::Pair::generate_with_phrase(None);
    println!("raw_seed: {:?}", HexDisplay::from(&raw_seed));
    println!("mnemonic: '{}'", mnemonic);
    println!("public: {:?}", HexDisplay::from(&pair.public().0));
    pair
}

pub fn hashing() {
    let data = "azz the pacchia!'";
    println!("data to be hashed: '{data}");
    println!("sha2_256: {:?}", HexDisplay::from(&sha2_256(data.as_bytes())));
    println!("blake2_128: {:?}", HexDisplay::from(&blake2_128(data.as_bytes())));
    println!("blake2_256: {:?}", HexDisplay::from(&blake2_256(data.as_bytes())));
    println!("keccak_256: {:?}", HexDisplay::from(&keccak_256(data.as_bytes())));
}

pub fn signing(pair: &sr25519::Pair) {
    println!("pair public key: {:?}", HexDisplay::from(&pair.public().0));
    let msg = "I am the message";
    println!("signing: '{msg}'");
    let s = pair.sign(msg.as_bytes());
    println!("signature: {:?}", HexDisplay::from(&s.0));
}