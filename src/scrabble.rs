use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashMap;
use tokio::task;

lazy_static! {
    static ref DICTIONARY: HashMap<&'static str, &'static str> = {
        let mut dict = HashMap::new();
        for line in DATA.lines() {
            let full_text: Vec<&str> = line.split(":").collect();
            let words = full_text[1];
            let alpha = full_text[0];
            dict.insert(alpha, words);
        }
        dict
    };
    static ref DATA: &'static str = include_str!("data");
}

#[derive(Debug)]
pub struct Answer {
    pub tiles: Vec<char>,
    pub longest_valid_words: Vec<String>,
    pub highest_scoring_words: Vec<String>,
    pub valid_words: Vec<String>,
}

pub fn sanitize(player_tiles: Vec<char>) -> (Vec<char>, bool) {
    let flag_good = player_tiles.len() <= 15;
    let clean_tiles: Vec<char> = player_tiles
        .into_iter()
        .filter(|&tile| tile.is_alphabetic())
        .map(|tile| tile.to_ascii_lowercase())
        .collect();
    (clean_tiles, flag_good)
}

async fn run_tiles(player_tiles: Vec<char>, use_full_subsets: bool) -> Answer {
    let number_of_tiles: u8 = player_tiles.len() as u8;

    let (mut player_tiles, flag) = sanitize(player_tiles);

    if !flag {
        return Answer {
            tiles: player_tiles.clone(),
            longest_valid_words: vec!["Your input must be less than 15 words".to_string()],
            highest_scoring_words: vec!["invalid input".to_string()],
            valid_words: vec!["invalid input".to_string()],
        };
    }

    player_tiles.sort();

    let mut subsets = produce_tile_subsets(player_tiles.clone(), number_of_tiles);
    subsets.dedup();

    let restricted_subsets: Vec<String> = if use_full_subsets {
        subsets
            .iter()
            .filter(|&sub| sub.len() == player_tiles.len())
            .cloned()
            .collect()
    } else {
        subsets
    };

    let results_list_future = task::spawn(async move { make_results_list(restricted_subsets) });

    let mut results_list = results_list_future.await.unwrap();
    results_list.dedup();

    let answer_future = task::spawn(async move { build(&mut results_list, player_tiles.clone()) });
    answer_future.await.unwrap()
}

pub async fn anagram_run(player_tiles: Vec<char>) -> Answer {
    run_tiles(player_tiles, true).await
}

pub async fn cheat_run(player_tiles: Vec<char>) -> Answer {
    run_tiles(player_tiles, false).await
}

pub async fn demo_run() -> Answer {
    let number_of_tiles: u8 = 7;
    let player_tiles = make_a_set_of_random_tiles(number_of_tiles);
    run_tiles(player_tiles, false).await
}

// #[derive(Debug)]
// pub struct Answer {
//     pub tiles: Vec<char>,
//     pub longest_valid_words: Vec<String>,
//     pub highest_scoring_words: Vec<String>,
//     pub valid_words: Vec<String>,
// }
//
// pub fn sanitize(player_tiles: Vec<char>) -> (Vec<char>, bool) {
//     let mut flag_good = true;
//     if player_tiles.len() > 15 {
//         flag_good = false;
//     }
//     let mut clean_tiles: Vec<char> = Vec::new();
//     for tile in player_tiles {
//         if tile.is_alphabetic() {
//             clean_tiles.push(tile.to_ascii_lowercase());
//         }
//     }
//     (clean_tiles, flag_good)
// }
//
// pub async fn anagram_run(player_tiles: Vec<char>) -> Answer {
//     let number_of_tiles: u8 = player_tiles.len() as u8;
//
//     let (mut player_tiles, flag) = sanitize(player_tiles);
//
//     if !flag {
//         return Answer {
//             tiles: player_tiles.to_vec(),
//             longest_valid_words: vec!["Your input must be less than 15 words".to_string()],
//             highest_scoring_words: vec!["invalid input".to_string()],
//             valid_words: vec!["invalid input".to_string()],
//         };
//     }
//
//     player_tiles.sort();
//
//     let mut subsets = produce_tile_subsets(player_tiles.clone(), number_of_tiles);
//     subsets.dedup();
//
//     let mut restricted_subsets: Vec<String> = Vec::new();
//
//     for sub in subsets.clone().iter() {
//         if sub.len() == player_tiles.clone().len() {
//             restricted_subsets.push(sub.to_string());
//         }
//     }
//
//     let results_list_future = task::spawn(async move { make_results_list(restricted_subsets) });
//
//     let mut results_list = results_list_future.await.unwrap();
//     results_list.dedup();
//
//     let answer_future = task::spawn(async move { build(&mut results_list, player_tiles.clone()) });
//     answer_future.await.unwrap()
// }
// pub async fn cheat_run(player_tiles: Vec<char>) -> Answer {
//     let number_of_tiles: u8 = player_tiles.len() as u8;
//
//     let (mut player_tiles, flag) = sanitize(player_tiles);
//
//     if !flag {
//         return Answer {
//             tiles: player_tiles.to_vec(),
//             longest_valid_words: vec!["Your input must be less than 15 words".to_string()],
//             highest_scoring_words: vec!["invalid input".to_string()],
//             valid_words: vec!["invalid input".to_string()],
//         };
//     }
//
//     player_tiles.sort();
//
//     let mut subsets = produce_tile_subsets(player_tiles.clone(), number_of_tiles);
//     subsets.dedup();
//
//     let results_list_future = task::spawn(async move { make_results_list(subsets) });
//
//     let mut results_list = results_list_future.await.unwrap();
//     results_list.dedup();
//
//     let answer_future = task::spawn(async move { build(&mut results_list, player_tiles.clone()) });
//     answer_future.await.unwrap()
// }
//
// pub async fn demo_run() -> Answer {
//     let number_of_tiles: u8 = 7;
//     let player_tiles = make_a_set_of_random_tiles(number_of_tiles);
//     let mut subsets = produce_tile_subsets(player_tiles.clone(), number_of_tiles);
//     subsets.dedup();
//
//     let results_list_future = task::spawn(async move { make_results_list(subsets) });
//
//     let mut results_list = results_list_future.await.unwrap();
//
//     results_list.dedup();
//
//     let answer_future = task::spawn(async move { build(&mut results_list, player_tiles.clone()) });
//     answer_future.await.unwrap()
// }

fn build(results_list: &mut [String], player_tiles: Vec<char>) -> Answer {
    if results_list.is_empty() {
        return Answer {
            tiles: player_tiles.to_vec(),
            longest_valid_words: vec!["No words found".to_string()],
            highest_scoring_words: vec!["No words found".to_string()],
            valid_words: vec!["No words found".to_string()],
        };
    };

    results_list.sort_by_key(|a| (a.len()));
    results_list.reverse();

    let mut longest_words: Vec<String> = Vec::new();
    let mut highest_words: Vec<String> = Vec::new();

    let longest_word_length = &results_list[0].len();

    for item in results_list.iter() {
        if item.len() < *longest_word_length {
            continue;
        } else {
            longest_words.push(item.to_string());
        }
    }

    let mut max_score: usize = 0;
    for item in results_list.iter() {
        if score_word(item.to_string()) > max_score {
            max_score = score_word(item.to_string());
        }
    }

    highest_words.push(format!("{max_score}"));
    for item in results_list.iter() {
        if score_word(item.to_string()) == max_score {
            highest_words.push(item.to_string());
        }
    }

    Answer {
        tiles: player_tiles.to_vec(),
        longest_valid_words: longest_words,
        highest_scoring_words: highest_words,
        valid_words: results_list.to_vec(),
    }
}

pub fn make_a_set_of_random_tiles(number_of_tiles: u8) -> Vec<char> {
    let mut bag = Vec::new();
    const ASCIISET: &[u8] =
        b"aaaaaaaaaiiiiiiiiioooooooonnnnnnrrrrrrttttttllllssssuuuuddddgggbbccmmppffhhvvwwyykjxqz";
    for _ in 1..(number_of_tiles + 1) {
        let idx = rand::thread_rng().gen_range(0..ASCIISET.len());
        bag.push(ASCIISET[idx] as char)
    }
    bag.sort();
    bag
}

fn score_word(word: String) -> usize {
    word.chars()
        .map(|letter| match letter {
            'e' | 'a' | 'i' | 'o' | 'n' | 'r' | 't' | 'l' | 's' | 'u' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0,
        })
        .sum()
}

fn produce_tile_subsets(player_tiles: Vec<char>, number_of_tiles: u8) -> Vec<String> {
    let quantity_of_subsets: usize = 2_usize.pow(number_of_tiles.into()) - 1_usize;
    let mut subsets: Vec<String> = Vec::with_capacity(quantity_of_subsets);

    for n in 0..1 << number_of_tiles {
        let mut subset = String::with_capacity(number_of_tiles.into());
        for (idx, tile) in player_tiles.iter().enumerate() {
            if n & (1 << idx) != 0 {
                subset.push(*tile);
            }
        }
        if !subset.is_empty() {
            subsets.push(subset);
        }
    }

    subsets.dedup();
    subsets.sort();
    subsets
}

fn make_results_list(subsets: Vec<String>) -> Vec<String> {
    let mut results_list: Vec<String> = Vec::new();

    for item in subsets {
        if let Some(word) = DICTIONARY.get(item.as_str()) {
            let each_word: Vec<_> = word.split(" ").collect();
            for item in each_word {
                results_list.push(item.trim().to_string());
            }
        }
    }

    results_list.dedup();
    results_list
}

