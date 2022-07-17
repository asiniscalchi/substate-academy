//! Drawing cards using VRFs

extern crate schnorrkel;
use std::sync::mpsc::RecvError;

mod player;
mod vrfcore;

use merlin::Transcript;
use rand_core::OsRng;
use schnorrkel::{
    vrf::{VRFInOut, VRFPreOut, VRFProof},
    Keypair, PublicKey,
};

const NUM_DRAWS: u8 = 8;
const NUM_CARDS: u16 = 52;

pub fn run() {
    let n_players = 10;
    println!("create the identities for {}", n_players);

    // register the players
    let mut players = Vec::new();
    for i in 0..n_players {
        let player = player::Player::new();
        let pk = player.public();
        players.push((pk, player));
        println!("player {i} pk {:x?}", pk.as_ref());
    }

    // generate the VRF seed
    let VRF_seed = generate_VRF_seed();

    // give players the cards
    for i in 0..players.len() {
        players[i].1.draw(&VRF_seed);
    }

    let counter = players[0].1.count_cards();
    for i in 0..counter {
        println!("[card {i}]: {}", players[0].1.reveal_one().unwrap().0);
    }
    // ok time to reveal the cards
    // let winner = (players[0], None);

    // for i in 0..players.len() {
    //     let pk = players[i].0;
    //     let signature = players[i].1.reveal_choosen_card().1;
    //     let revealed_card = recieve(&pk, &signature, &VRF_seed);
    //     println!("player {i} reveal card {:?}", revealed_card);
    // }
}

fn generate_VRF_seed() -> [u8; 32] {
    [0u8; 32]
}

/// Processes VRF inputs, checking validity of the number of draws
fn draw_transcript(seed: &[u8; 32], draw_num: u8) -> Option<Transcript> {
    if draw_num > NUM_DRAWS {
        return None;
    }
    let mut t = Transcript::new(b"Card Draw Transcript");
    t.append_message(b"seed", seed);
    t.append_u64(b"draw", draw_num as u64);
    Some(t)
}

/// Computes actual card draw from VRF inputs & outputs together
fn find_card(io: &VRFInOut) -> Option<u16> {
    let b: [u8; 8] = io.make_bytes(b"card");
    // We make one in half the draws invalid so nobody knows how many cards anyone else has
    // if b[7] & 0x80 { return None; }
    Some((u64::from_le_bytes(b) % (NUM_CARDS as u64)) as u16)
}

/// Attempts to draw a card
fn try_draw(keypair: &Keypair, seed: &[u8; 32], draw_num: u8) -> Option<(u16, [u8; 97])> {
    let t = draw_transcript(seed, draw_num)?;
    let (io, proof, _) = keypair.vrf_sign(t);
    let card = find_card(&io)?;
    let mut vrf_signature = [0u8; 97];
    // the first 32 bytes are io
    vrf_signature[..32].copy_from_slice(&io.to_preout().to_bytes()[..]);
    // the next 64 bytes are the proof
    vrf_signature[32..96].copy_from_slice(&proof.to_bytes()[..]);
    // the final byte is the draw number
    vrf_signature[96] = draw_num;
    Some((card, vrf_signature))
}

/// Draws all our cards for the give seed
fn draws(keypair: &Keypair, seed: &[u8; 32]) -> Vec<(u16, [u8; 97])> {
    (0..NUM_DRAWS)
        .filter_map(|i| try_draw(keypair, seed, i))
        .collect()
}

/// Verifies a card play
///
/// We depend upon application code to enforce the public key and seed
/// being chosen correctly.
///
/// We encode the draw number into the vrf signature since an honest
/// application has no use for this, outside the verification check in
/// `draw_transcript`.
fn recieve(public: &PublicKey, vrf_signature: &[u8; 97], seed: &[u8; 32]) -> Option<u16> {
    let t = draw_transcript(seed, vrf_signature[96])?;
    let out = VRFPreOut::from_bytes(&vrf_signature[..32]).ok()?;
    let proof = VRFProof::from_bytes(&vrf_signature[32..96]).ok()?;
    // We need not understand the error type here, but someone might
    // care about invalid signatures vs invalid card draws.
    let (io, _) = public.vrf_verify(t, &out, &proof).ok()?;
    find_card(&io)
}
