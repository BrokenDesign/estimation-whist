mod model;
mod strat;

use env_logger::{Builder, Env};
use log::{debug, error, info, warn};
use model::{Card, CardSuit, CardValue, Deck, Game, Hand, Player};
use rand::Rng;

fn init_logging(level: &str) {
    std::env::set_var("RUST_LOG", level);
    Builder::from_env(Env::default().default_filter_or(level))
        .target(env_logger::Target::Stdout)
        .init();
}

// TODO: load players from a file
fn load_players() -> Vec<Player> {
    vec![
        Player::new(
            Hand::new(Vec::new(), CardSuit::Clubs),
            0,
            0,
            strat::random_card,
            strat::random_bid,
        ),
        Player::new(
            Hand::new(Vec::new(), CardSuit::Clubs),
            0,
            0,
            strat::random_card,
            strat::random_bid,
        ),
        Player::new(
            Hand::new(Vec::new(), CardSuit::Clubs),
            0,
            0,
            strat::random_card,
            strat::random_bid,
        ),
    ]
}

fn main() {
    init_logging("debug");
    for i in 0..10 {
        let players = load_players();
        let mut game = Game::new(players);
        game.play();
        log::info!("FINAL SCORES: {:?}", game.scores);
    }
}
