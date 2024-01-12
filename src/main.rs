mod base;
mod game;
mod strat;

use base::{Card, CardSuit, CardValue, Deck};
use env_logger::{Builder, Env};
use game::{Game, Hand, Player};
use log::{debug, error, info, warn};
use rand::Rng;

use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io::Read;

fn init_logging(level: &str) {
    std::env::set_var("RUST_LOG", level);
    Builder::from_env(Env::default().default_filter_or(level))
        .target(env_logger::Target::Stdout)
        .init();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerConfig {
    pub name: String,
    pub play_strategy: String,
    pub bid_strategy: String,
}

fn load_yaml_file(file_path: &str) -> Result<Vec<PlayerConfig>, Box<dyn std::error::Error>> {
    let mut file = File::open("players.yaml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let players: Vec<PlayerConfig> = serde_yaml::from_str(&content)?;
    Ok(players)
}

fn config_to_players(config: Vec<PlayerConfig>) -> Vec<Player> {
    let mut players = Vec::new();
    for player_config in config {
        let play_strategy = match player_config.play_strategy.as_str() {
            "random" => strat::random_card,
            "highest" => panic!("Highest card strategy not implemented"),
            "lowest" => panic!("Lowest card strategy not implemented"),
            _ => panic!("Invalid play strategy"),
        };

        let bid_strategy = match player_config.bid_strategy.as_str() {
            "random" => strat::random_bid,
            "optimistic" => panic!("Optimistic bid strategy not implemented"),
            "pessimistic" => panic!("Pessimistic bid strategy not implemented"),
            _ => panic!("Invalid bid strategy"),
        };

        let player = Player::new(
            player_config.name,
            Hand::new(Vec::new(), CardSuit::Clubs),
            0,
            0,
            play_strategy,
            bid_strategy,
        );
        players.push(player);
    }
    players
}

fn load_players() -> Vec<Player> {
    let file_path = "players.yaml";
    match load_yaml_file(file_path) {
        Ok(config) => config_to_players(config),
        Err(err) => panic!("Error loading YAML file: {}", err),
    }
}

fn main() {
    init_logging("debug");
    let players = load_players();
    println!("{:?}", players);
    // for i in 0..10 {
    //     let players = load_players();
    //     let mut game = Game::new(players);
    //     game.play();
    //     log::info!("FINAL SCORES: {:?}", game.scores);
    // }
}
