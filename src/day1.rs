use sp_core::{crypto::Derive, hashing::sha2_256, hexdisplay::*, *};

pub fn pair() -> sr25519::Pair {
    let (pair, mnemonic, raw_seed) = sr25519::Pair::generate_with_phrase(None);
    println!("raw_seed: {:?}", HexDisplay::from(&raw_seed));
    println!("mnemonic: '{}'", mnemonic);
    println!("public: {:?}", HexDisplay::from(&pair.public().0));
    pair
}

pub fn hashing() {
    let data = "azz the pacchia!'";
    println!("data to be hashed: '{data}");
    println!(
        "sha2_256: {:?}",
        HexDisplay::from(&sha2_256(data.as_bytes()))
    );
    println!(
        "blake2_128: {:?}",
        HexDisplay::from(&blake2_128(data.as_bytes()))
    );
    println!(
        "blake2_256: {:?}",
        HexDisplay::from(&blake2_256(data.as_bytes()))
    );
    println!(
        "keccak_256: {:?}",
        HexDisplay::from(&keccak_256(data.as_bytes()))
    );
}

pub fn signing(pair: &sr25519::Pair) {
    println!("pair public key: {:?}", HexDisplay::from(&pair.public().0));
    let msg = "I am the message";
    println!("signing: '{msg}'");
    let s = pair.sign(msg.as_bytes());
    println!("signature: {:?}", HexDisplay::from(&s.0));
    let verified = sr25519::Pair::verify(&s, msg, &pair.public());
    println!("verified: {}", verified);
}

pub fn derive() {
    let (pair, raw_seed) = sr25519::Pair::generate();
    println!("pair public key: {:?}", HexDisplay::from(&pair.public().0));
    let hard_derived = pair
        .derive(Some(DeriveJunction::hard(&b"foo"[..])).into_iter(), None)
        .unwrap();
    println!(
        "hard_derived: {:?}",
        HexDisplay::from(&hard_derived.0.public().0)
    );
    let hard_derived1 = sr25519::Pair::from_string_with_seed(
        &format!("0x{}//foo", HexDisplay::from(&raw_seed)),
        None,
    )
    .unwrap();
    println!(
        "hard_derived1: {:?}",
        HexDisplay::from(&hard_derived1.0.public().0)
    );

    let data = "this is my message";
    let signature = pair.sign(data.as_bytes());
    let public = pair.public();
    let soft_derived = public
        .derive(Some(DeriveJunction::soft(&b"foo"[..])).into_iter())
        .unwrap();
    println!("soft derived: {}", soft_derived);

    let verified = sr25519::Pair::verify(&signature, data.as_bytes(), &soft_derived);
    println!("verified with soft derived public key {}", verified); // should be false
}
