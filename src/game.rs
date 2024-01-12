use crate::base::{Card, CardSuit, CardValue, Deck};
use log::{debug, error, info, warn};
use rand::Rng;
use std::collections::HashMap;

// ------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub trumps: CardSuit,
}

impl Hand {
    pub fn new(cards: Vec<Card>, trumps: CardSuit) -> Hand {
        Hand {
            cards: cards,
            trumps: trumps,
        }
    }
}

// ------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
    pub estimate: usize,
    pub tricks: usize,
    pub play_strategy: fn(&mut Self, &Vec<Card>) -> Card,
    pub estimate_strategy: fn(&mut Self, usize, &Vec<usize>) -> usize,
}

impl Player {
    pub fn new(
        name: String,
        hand: Hand,
        estimate: usize,
        tricks: usize,
        play_strategy: fn(&mut Self, &Vec<Card>) -> Card,
        estimate_strategy: fn(&mut Self, usize, &Vec<usize>) -> usize,
    ) -> Player {
        Player {
            name: name,
            hand: hand,
            estimate: estimate,
            tricks: tricks,
            play_strategy: play_strategy,
            estimate_strategy: estimate_strategy,
        }
    }

    pub fn play(&mut self, played_cards: &Vec<Card>) -> Card {
        (self.play_strategy)(self, played_cards)
    }

    pub fn estimate(&mut self, num_cards: usize, estimates: &Vec<usize>) -> usize {
        (self.estimate_strategy)(self, num_cards, estimates)
    }
}

#[derive(Debug)]
pub struct Round<'a> {
    deck: Deck,
    players: &'a mut Vec<Player>,
    num_cards: usize,
    trumps: CardSuit,
    estimates: HashMap<Player, usize>,
    scores: HashMap<Player, usize>,
    played_cards: Vec<Card>,
}

impl<'a> Round<'a> {
    pub fn new(players: &'a mut Vec<Player>, num_cards: usize, trumps: CardSuit) -> Round<'a> {
        Round {
            deck: Deck::new(),
            players: players,
            num_cards: num_cards,
            trumps: trumps,
            estimates: HashMap::new(),
            scores: HashMap::new(),
            played_cards: Vec::new(),
        }
    }

    pub fn deal(&mut self) {
        let hands = self.deck.deal(&mut self.players.len(), &mut self.num_cards);
        let mut i = 0;
        for hand in hands {
            self.players.get_mut(i).unwrap().hand = Hand::new(hand, self.trumps.clone());
            i += 1;
        }
    }

    pub fn bid(&mut self) {
        for i in 0..self.players.len() {
            let mut previous = Vec::new();
            let estimate = self.players[i].estimate(self.num_cards, &previous);
            self.estimates
                .insert(self.players.get(i).unwrap().clone(), estimate);
            previous.push(estimate);
        }
    }

    fn find_winner(&self, trick: &Vec<Card>) -> usize {
        let lead_suit = trick.get(0).unwrap().suit;
        let mut winning_card = trick.get(0).unwrap();
        let mut winning_player = 0;
        for (player, card) in trick.iter().enumerate() {
            if card.suit == self.trumps {
                if winning_card.suit != self.trumps {
                    winning_card = card;
                    winning_player = player;
                } else if card.value > winning_card.value {
                    winning_card = card;
                    winning_player = player;
                }
            } else if card.suit == lead_suit {
                if winning_card.suit != self.trumps && card.value > winning_card.value {
                    winning_card = card;
                    winning_player = player;
                }
            }
        }
        winning_player
    }

    fn play_trick(&mut self) {
        let mut trick = Vec::new();
        for i in 0..self.players.len() {
            let card = self.players[i].play(&self.played_cards);
            trick.push(card.clone());
            self.played_cards.push(card.clone());
        }
        let winner = self.find_winner(&trick);
        self.players[winner].tricks += 1;
        self.players.rotate_left(winner)
    }

    pub fn play(&mut self) {
        for i in 0..self.num_cards {
            self.play_trick();
        }
    }

    pub fn score(&mut self) {
        for i in 0..self.players.len() {
            let player = self.players.get(i).expect("Player not found");
            let estimate = self.estimates.get(player).expect("Player not found");
            let tricks = player.tricks;
            let score = tricks + (*estimate == tricks) as usize * 10;
            self.scores.insert(player.clone(), score);
        }
    }

    pub fn evaluate(&mut self) {
        self.deal();
        self.bid();
        self.play();
        self.score();
    }
}

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub scores: HashMap<Player, usize>,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Game {
        Game {
            players: players.clone(),
            scores: players.iter().map(|p| (p.clone(), 0 as usize)).collect(),
        }
    }

    pub fn play_round(&mut self, num_cards: usize, trumps: CardSuit) {
        let mut round = Round::new(&mut self.players, num_cards, trumps);
        round.evaluate();
        for (player, score) in round.scores {
            let mut current_score = self.scores.get_mut(&player).expect("Player not found");
            *current_score += score;
        }
    }

    pub fn play(&mut self) {
        let suits = [
            CardSuit::Clubs,
            CardSuit::Diamonds,
            CardSuit::Hearts,
            CardSuit::Spades,
        ];
        for i in 0..8 {
            let num_cards = 8 - i as usize;
            let trumps = suits[i % 4];
            let round = self.play_round(num_cards, trumps);
        }
    }
}
