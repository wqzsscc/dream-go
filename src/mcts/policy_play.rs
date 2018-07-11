// Copyright 2018 Karl Sundequist Blomdahl <karl.sundequist.blomdahl@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ordered_float::OrderedFloat;

use go::{Board, Color};
use util::config;
use mcts::*;

/// pick the move from the policy by taking the top 80% of the policy, and then
/// choosing a move from the remains (weighted by the moves policy value)
/// 
/// # Arguments
/// 
/// * `policy` - 
/// * `temperature` - 
/// 
fn policy_choose(policy: &[f32], temperature: f32) -> Option<usize> {
    let mut candidates = (0..362).collect::<Vec<usize>>();
    let mut subtotals = [0.0f32; 362];

    candidates.sort_unstable_by_key(|&i| OrderedFloat(-policy[i]));

    // calculate the subtotals for the sorted candidates so that we can
    // efficiently determine the cutoff point, using binary search.
    for i in 0..362 {
        let j = candidates[i];
        let value = policy[j].powf(temperature);

        if value.is_finite() {
            subtotals[i] = if i > 0 { subtotals[i-1] + value } else { value };
        } else {
            subtotals[i] = if i > 0 { subtotals[i-1] } else { 0.0 };
        }
    }

    // if there are no valid moves remaining then pass, otherwise pick
    // a random move using binary search over the `subtotals`.
    if subtotals[361] > 0.0 {
        let threshold = 0.8 * thread_rng().gen::<f32>() * subtotals[361];
        let mut index = match subtotals.binary_search_by_key(&OrderedFloat(threshold), |&s| OrderedFloat(s)) {
            Ok(i) => i,
            Err(i) => i
        };

        // if the binary search found one of the invalid moves then step
        // backward, and then forward again iff we landed on a move with
        // no preceeding legal move.
        while index > 0 && subtotals[index - 1] == subtotals[index] {
            index -= 1;
        }

        while subtotals[index] == 0.0 {
            index += 1;
        }

        Some(candidates[index])
    } else {
        None
    }
}

/// Play a game against the engine and return the result of the game.
/// This is different from `self_play` because this method does not
/// perform any search and only plays stochastically according
/// to the policy network.
/// 
/// # Arguments
/// 
/// * `server` - the server to use during evaluation
/// 
fn policy_play_one(server: &PredictGuard) -> GameResult {
    let mut temperature = (*config::TEMPERATURE + 1e-3).recip();
    let mut sgf = String::new();

    // loop until we run or of legal moves, the board is fully scoreable, or
    // we have played 722 moves in total.
    let mut board = Board::new(get_random_komi());
    let mut color = Color::Black;
    let mut pass_count = 0;

    while pass_count < 2 && board.count() < 722 {
        let result = forward(&server, &board, color);
        let index = if let Some((_value, policy)) = result {
            match policy_choose(&policy, temperature) {
                Some(index) => index,
                None => 361
            }
        } else {
            break;  // failure?
        };

        if index == 361 {
            sgf += &format!(";{}[]", color);
            pass_count += 1;
        } else {
            let (x, y) = (tree::X[index] as usize, tree::Y[index] as usize);

            board.place(color, x, y);
            sgf += &format!(";{}[{}]", color, Sabaki::to_sgf(x, y));
            pass_count = 0;
        }

        temperature = min(5.0, 1.03 * temperature);
        color = color.opposite();
    }

    GameResult::Ended(sgf, board)
}

/// Play games against the engine and return the results of the game over
/// the returned channel. This is different from `self_play` because this
/// method does not perform any search and only plays stochastically according
/// to the policy network.
/// 
/// # Arguments
/// 
/// * `network` - the neural network to use during evaluation
/// * `num_games` - 
/// 
pub fn policy_play(network: Network, num_games: usize) -> (Receiver<GameResult>, PredictService) {
    let server = predict::service(network);
    let (sender, receiver) = channel();

    // spawn the worker threads that generate the self-play games
    let num_workers = ::std::cmp::min(*config::NUM_GAMES, num_games);
    let remaining = Arc::new(AtomicUsize::new(num_games));

    for _ in 0..num_workers {
        let remaining = remaining.clone();
        let sender = sender.clone();
        let server = server.lock().clone_static();

        thread::spawn(move || {
            while remaining.load(Ordering::Acquire) > 0 {
                remaining.fetch_sub(1, Ordering::AcqRel);

                let result = policy_play_one(&server);

                if sender.send(result).is_err() {
                    break
                }
            }
        });
    }

    (receiver, server)
}
