use crate::model::{Card, CardSuit, CardValue, Deck, Game, Hand, Player};
use log::{debug, error, info, warn};
use rand::Rng;

pub fn random_card(player: &mut Player, played_cards: &Vec<Card>) -> Card {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..player.hand.cards.len());
    let card = player.hand.cards[index];
    player.hand.cards.remove(index);
    card
}

pub fn random_bid(player: &mut Player, num_cards: usize, bids: &Vec<usize>) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..num_cards) as usize
}
