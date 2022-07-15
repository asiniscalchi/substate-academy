
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

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn initial_draw() {
        let p = Player::new();
        let cards = p.reveal_all();
        assert_eq!(cards.len(), 0);
    }

    #[test]
    #[should_panic]
    fn ask_for_unexistent_card() {
         let p = Player::new();
        let card = p.reveal_choosen_card();
        assert_eq!(card.0, 0);
    }

    #[test] 
    fn draw_creates_8_cards() {
        let mut p = Player::new();
        p.draw(&[0u8; 32]);
        let cards = p.reveal_all();
        assert_eq!(cards.len(), 8);
    }
}
