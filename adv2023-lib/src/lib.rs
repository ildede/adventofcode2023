use wasm_bindgen::prelude::*;
use regex::Regex;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[wasm_bindgen]
pub fn day_2_part_a(input: &str) -> String {

    let result = input.lines()
        .into_iter()
        .map(to_game_info)
        .filter(|g| {
            println!("{:?}", g);
            is_possible(g)
        })
        .map(|g| g.id)
        .sum::<u32>();

    String::from(result.to_string())
}

fn is_possible(game: &GameInfo) -> bool {
    for show in game.shows.clone() {
        if show.red > MAX_RED {
            return false;
        }
        if show.green > MAX_GREEN {
            return false;
        }
        if show.blue > MAX_BLUE {
            return false;
        }
    }
    true
}

#[derive(Debug, PartialEq)]
struct GameInfo {
    id: u32,
    shows: Vec<Show>,
}

#[derive(Debug, PartialEq, Clone)]
struct Show {
    red: u32,
    green: u32,
    blue: u32,
}

fn to_game_info(line: &str) -> GameInfo {
    let modified_line = &format!("{};", line);
    let game_id = Regex::new(r"Game (?<id>\d+):").unwrap();
    let id: u32 = match game_id.captures(modified_line) {
        Some(caps) => *&caps["id"].parse().unwrap(),
        None => 0,
    };
    let get_shows = Regex::new(r".*?;").unwrap();
    let shows_raw: Vec<&str> = get_shows.find_iter(modified_line).map(|m| m.as_str()).collect();
    let red = Regex::new(r"(?<count>\d+)\sred").unwrap();
    let blue = Regex::new(r"(?<count>\d+)\sblue").unwrap();
    let green = Regex::new(r"(?<count>\d+)\sgreen").unwrap();

    let shows = shows_raw.into_iter()
        .map(|s| Show {
            red: red.captures(s).map(|caps| *&caps["count"].parse().unwrap()).unwrap_or_else(|| 0),
            green: green.captures(s).map(|caps| *&caps["count"].parse().unwrap()).unwrap_or_else(|| 0),
            blue: blue.captures(s).map(|caps| *&caps["count"].parse().unwrap()).unwrap_or_else(|| 0),
        })
        .collect();

    GameInfo {
        id,
        shows,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = day_2_part_a(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        );
        assert_eq!(result, "8");
    }

    #[test]
    fn translate_game_info() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let result = to_game_info(line);
        let expected = GameInfo {
            id: 1,
            shows: vec![
                Show { red: 4, green: 0, blue: 3 },
                Show { red: 1, green: 2, blue: 6 },
                Show { red: 0, green: 2, blue: 0 },
            ],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_possible() {
        let game = GameInfo {
            id: 1,
            shows: vec![
                Show { red: 4, green: 0, blue: 3 },
                Show { red: 1, green: 2, blue: 6 },
                Show { red: 0, green: 2, blue: 0 },
            ],
        };

        assert_eq!(is_possible(&game), true);
    }

    #[test]
    fn test_is_not_possible() {
        let game = GameInfo {
            id: 1,
            shows: vec![
                Show { red: 20, green: 0, blue: 3 },
                Show { red: 1, green: 2, blue: 6 },
                Show { red: 0, green: 2, blue: 0 },
            ],
        };

        assert_eq!(is_possible(&game), false);
    }

}
