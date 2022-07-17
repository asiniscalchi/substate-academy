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

    pub fn reveal_one(&mut self) -> Option<(u16, [u8; 97])> {
        if self.cards.len() == 0 {
            return None;
        }
        let max_value = self.cards.iter().max_by_key(|x| x.0).unwrap();
        let max_key = self.cards.iter().position(|x| x.0 == max_value.0).unwrap();
        Some(self.cards.remove(max_key))
    }

    pub fn count_cards(&self) -> usize {
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
        let mut p = Player::new();
        let card = p.reveal_one();
        assert_eq!(card, None);
    }

    #[test]
    fn when_card_is_revealed_less_card() {
        let mut p = Player::new();
        p.draw(&[0; 32]);
        assert_eq!(p.count_cards(), 8);
        p.reveal_one();
        assert_eq!(p.count_cards(), 7);
    }

    // #[test]
    // fn cards_are_revealed_starting_max() {
    //     let mut p = Player::new();
    //     p.draw(&[0; 32]);
    //     let counter = p.count_cards();
    //     assert_eq!(p.count_cards(), 8);
    //     for i in 0..counter {
    //         let card = p.reveal_one();
    //     }
    // }

    #[test]
    fn ask_for_a_card() {
        let mut p = Player::new();
        p.draw(&[0; 32]);
        let card = p.reveal_one();
        assert!(card != None);
    }
}
