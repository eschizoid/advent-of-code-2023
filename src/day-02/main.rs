use nom::bytes::complete::{take_until, take_while};
use nom::character::complete as cc;
use nom::combinator::{all_consuming, map_res};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{bytes::complete::tag, Finish, IResult};

fn main() {
  let games_1 = all_consuming(parse_all_games)(include_str!("input.txt"))
    .finish()
    .unwrap()
    .1;

  let games_2 = all_consuming(parse_all_games)(include_str!("input.txt"))
    .finish()
    .unwrap()
    .1;

  println!("Total: {:?}", solution_1(games_1));
  println!("Total: {:?}", solution_2(games_2));
}

fn solution_1(games: Vec<Game>) -> i32 {
  let max_red_value = 12;
  let max_green_value = 13;
  let max_blue_value = 14;
  let mut total = 0;

  games.iter().for_each(|game| {
    let mut all_sets_valid = true;
    game.sets.iter().for_each(|set| {
      let invalid_sets = set
        .dices
        .iter()
        .filter(|dice| match dice.color {
          Color::Red => dice.value > max_red_value,
          Color::Green => dice.value > max_green_value,
          Color::Blue => dice.value > max_blue_value,
        })
        .count();
      if invalid_sets > 0 {
        all_sets_valid = false;
      }
    });
    if all_sets_valid {
      total += game.game_id;
    }
  });
  return total;
}

fn solution_2(games: Vec<Game>) -> i32 {
  let mut total = 0;
  games.iter().for_each(|game| {
    let max_red_value = game
      .sets
      .iter()
      .flat_map(|set| set.dices.iter())
      .filter(|dice| dice.color == Color::Red)
      .max()
      .unwrap()
      .value;

    let max_blue_value = game
      .sets
      .iter()
      .flat_map(|set| set.dices.iter())
      .filter(|dice| dice.color == Color::Blue)
      .max()
      .unwrap()
      .value;

    let max_green_value = game
      .sets
      .iter()
      .flat_map(|set| set.dices.iter())
      .filter(|dice| dice.color == Color::Green)
      .max()
      .unwrap()
      .value;
    total += max_red_value * max_blue_value * max_green_value;
  });
  return total;
}

pub fn parse_all_games(i: &str) -> IResult<&str, Vec<Game>> {
  let game_list = separated_list1(cc::newline, parse_game)(i);
  return game_list;
}

fn parse_game(i: &str) -> IResult<&str, Game> {
  // Sample input:
  // Game 1: 1 green, 2 blue; 15 blue, 12 red, 2 green; 4 red, 6 blue; 10 blue, 8 red; 3 red, 12 blue; 1 green, 12 red, 8 blue

  let (i, (_, game_id)) = tuple((tag("Game "), take_until(":")))(i)?;
  let (i, (_, games)) = tuple((
    tag(": "),
    separated_list1(
      tag("; "),
      map_res(take_while(|c| c != ';' && c != '\n'), |s: &str| {
        s.parse::<String>()
      }),
    ),
  ))(i)?;

  Ok((
    i,
    Game {
      game_id: game_id.to_string().parse::<i32>().unwrap(),
      sets: games
        .iter()
        .enumerate()
        .map(|game| {
          let idx = game.0;
          let dices = game
            .1
            .split(", ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|pair| {
              let color = color_from_string(pair.split(" ").collect::<Vec<&str>>()[1]);
              let value = pair.split(" ").collect::<Vec<&str>>()[0].parse().unwrap();
              Dice { color, value }
            })
            .collect::<Vec<Dice>>();
          GameSet {
            dices,
            set_number: idx as i32,
          }
        })
        .collect::<Vec<GameSet>>(),
    },
  ))
}

fn color_from_string(color: &str) -> Color {
  match color {
    "red" => Color::Red,
    "green" => Color::Green,
    "blue" => Color::Blue,
    _ => panic!("Unknown color: {}", color),
  }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Game {
  pub game_id: i32,
  pub sets: Vec<GameSet>,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct GameSet {
  pub dices: Vec<Dice>,
  pub set_number: i32,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Dice {
  pub color: Color,
  pub value: i32,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Color {
  Red,
  Green,
  Blue,
}
