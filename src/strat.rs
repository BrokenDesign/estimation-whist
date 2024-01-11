use log::{debug, error, info, warn};
use rand::Rng;
use whist::model;

// fn random_card(player: &Player, played_cards: &Vec<Card>) -> Card {
//     let mut rng = rand::thread_rng();
//     let index = rng.gen_range(0..player.hand.cards.len());
//     player.hand.cards[index]
// }
// |player, played_cards| {
//                 let mut rng = rand::thread_rng();
//                 let index = rng.gen_range(0..player.hand.cards.len());
//                 player.hand.cards[index]
//             },
//         },
