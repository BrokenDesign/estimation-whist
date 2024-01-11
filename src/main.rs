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

fn load_players() {
    TODO!()
}

fn main() {
    init_logging("debug");
    let players = load_players(); 
    let mut game = Game::new(
        players: players
        num_cards: 13,
    );
    game.play();
    log::info!("FINAL SCORES: {:?}", game.scores);
}
