use rand_core::OsRng;
use schnorrkel::{Keypair, PublicKey};

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

    fn reveal_one(&self) -> Option<(u16, [u8; 97])> {
        if self.cards.len() == 0 {
            return None;
        }
        Some(self.cards[0])
    }

    fn count_cards(&self) -> usize {
        self.cards.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_draw() {
        let p = Player::new();
        assert_eq!(p.count_cards(), 0);
    }

    #[test]
    fn ask_for_unexistent_card() {
        let p = Player::new();
        let card = p.reveal_one();
        assert_eq!(card, None);
    }

    #[test]
    fn ask_for_a_card() {
        let mut p = Player::new();
        p.draw(&[0; 32]);
        let card = p.reveal_one();
        assert!(card != None);
    }
}
