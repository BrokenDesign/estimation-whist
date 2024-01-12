use rand::Rng;

/// Generic non-game specific cards stuff

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

// ------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuit,
}

impl Card {
    pub fn new(value: CardValue, suit: CardSuit) -> Card {
        Card {
            value: value,
            suit: suit,
        }
    }
}

// ------------------------------------------------------------------------

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
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

    fn random(&mut self) -> Card {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.cards.len());
        let card = self.cards.get(index).unwrap().clone();
        self.cards.remove(index);
        card
    }

    pub fn deal(&mut self, num_players: &usize, num_cards: &usize) -> Vec<Vec<Card>> {
        let mut hands = Vec::new();
        for _ in 0..*num_players {
            let mut hand = Vec::new();
            for _ in 0..*num_cards {
                let card = self.random();
                hand.push(card);
            }
            hands.push(hand);
        }
        hands
    }
}
