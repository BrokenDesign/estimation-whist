use log::{debug, error, info, warn};
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    value: CardValue,
    suit: CardSuit,
}

impl Card {
    pub fn new(value: CardValue, suit: CardSuit) -> Card {
        Card {
            value: value,
            suit: suit,
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        let suits = [
            CardSuit::Clubs,
            CardSuit::Diamonds,
            CardSuit::Hearts,
            CardSuit::Spades,
        ];
        let values = [
            CardValue::Two,
            CardValue::Three,
            CardValue::Four,
            CardValue::Five,
            CardValue::Six,
            CardValue::Seven,
            CardValue::Eight,
            CardValue::Nine,
            CardValue::Ten,
            CardValue::Jack,
            CardValue::Queen,
            CardValue::King,
            CardValue::Ace,
        ];

        for suit in suits {
            for value in values {
                cards.push(Card::new(value, suit));
            }
        }

        Deck { cards: cards }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_all(&mut self, cards: Vec<Card>) {
        for card in cards {
            self.cards.push(card);
        }
    }

    pub fn remove_card(&mut self, card: Card) {
        let index = self
            .cards
            .iter()
            .position(|&x| x == card)
            .expect("Card not found");
        self.cards.remove(index);
    }

    pub fn select_random(&mut self) -> Card {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.cards.len());
        let card = self.cards[index];
        self.cards.remove(index);
        card
    }

    pub fn deal(&mut self, players: &mut Vec<Player>, num_cards: &usize) {
        for player in players {
            assert!(player.hand.cards.is_empty(), "Player already has cards");
            for _ in 0..*num_cards {
                let card = self.select_random();
                player.hand.cards.push(card);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    trumps: CardSuit,
}

impl Hand {
    pub fn new(cards: Vec<Card>, trumps: CardSuit) -> Hand {
        Hand {
            cards: cards,
            trumps: trumps,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove_card(&mut self, card: Card) {
        let index = self
            .cards
            .iter()
            .position(|&x| x == card)
            .expect("Card not found");
        self.cards.remove(index);
    }

    pub fn select_random(&mut self) -> Card {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.cards.len());
        let card = self.cards[index];
        self.cards.remove(index);
        card
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    hand: Hand,
    estimate: usize,
    tricks: usize,
    play_strategy: fn(&Self, &Vec<Card>) -> Card,
    estimate_strategy: fn(&Self, usize, Vec<usize>) -> usize,
}

impl Player {
    pub fn new(
        hand: Hand,
        estimate: usize,
        tricks: usize,
        play_strategy: fn(&Self, &Vec<Card>) -> Card,
        estimate_strategy: fn(&Self, usize, Vec<usize>) -> usize,
    ) -> Player {
        Player {
            hand: hand,
            estimate: estimate,
            tricks: tricks,
            play_strategy: play_strategy,
            estimate_strategy: estimate_strategy,
        }
    }

    fn is_valid_play(&mut self, card: Card, trick: &Vec<Card>) -> bool {
        TODO!()
    }

    fn is_valid_estimate(&mut self, estimate: usize, estimates: &Vec<usize>) -> bool {
        TODO!()
    }

    pub fn play(&mut self, played_cards: &Vec<Card>) -> Card {
        (self.play_strategy)(self, played_cards)
    }

    pub fn estimate(&mut self, num_cards: usize, estimates: Vec<usize>) -> usize {
        (self.estimate_strategy)(self, num_cards, estimates)
    }
}

#[derive(Debug)]
pub struct Round<'a> {
    deck: Deck,
    players: &'a mut Vec<Player>,
    num_cards: usize,
    played_cards: Vec<Card>,
    trumps: CardSuit,
}

impl<'a> Round<'a> {
    pub fn new(players: &'a mut Vec<Player>, num_cards: usize, trumps: CardSuit) -> Round<'a> {
        Round {
            deck: Deck::new(),
            players,
            num_cards: num_cards,
            played_cards: Vec::new(),
            trumps: trumps,
        }
    }

    pub fn deal(&mut self) {
        self.deck.deal(&mut self.players, &mut self.num_cards);
    }

    pub fn play(&mut self) {
        log::debug!("Playing round({:?})", self.num_cards);
        
        for i in 0..self.players.len() {
            log::debug!("{}...", i);
            self.players[i].hand.cards = Vec::new();
        }
    }
}

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    scores: Vec<usize>,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Game {
        Game {
            scores: vec![0; players.len()],
            players: players,
        }
    }

    pub fn play_round(&mut self, num_cards: usize, trumps: CardSuit) {
        let mut round = Round::new(&mut self.players, num_cards, trumps);
        round.deal();
        round.play();
        for (i, player) in round.players.iter().enumerate() {
            self.scores[i] += player.tricks + (player.estimate == player.tricks) as usize * 10;
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
