
use rand_core::OsRng;
use schnorrkel::{
    Keypair, PublicKey,
};

use super::vrfcore;

pub struct Player {
    keypair: Keypair,
    cards: Vec<(u16, [u8; 97])>,
}

impl Player {
    pub fn new() -> Self {
        let csprng = rand_core::OsRng;
        let keypair = Keypair::generate_with(csprng);
        Player {
            keypair: keypair,
            cards: Vec::new(),
        }
    }

    pub fn draw(&mut self, seed: &[u8; 32]) {
        self.cards = vrfcore::draws(&self.keypair, seed);
    }

    pub fn public(&self) -> PublicKey {
        self.keypair.public
    }

    fn signature(&self, idx: usize) -> [u8; 97] {
        self.cards[idx].1
    }

    fn reveal_choosen_card(&self) -> (u16, [u8; 97]) {
        self.cards[0]
    }

    fn reveal_all(&self) -> Vec<(u16, [u8; 97])> {
        self.cards.clone()
    }
}
