use nom::bytes::complete::{take_until, take_while};
use nom::character::complete as cc;
use nom::combinator::{all_consuming, map_res};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{bytes::complete::tag, Finish, IResult};

fn main() {
  let games = all_consuming(parse_all_games)(include_str!("input.txt"))
    .finish()
    .unwrap()
    .1;

  println!("Games: {:?}", games);
}

pub fn parse_all_games(i: &str) -> IResult<&str, Vec<Game>> {
  let game_list = separated_list1(cc::newline, parse_game)(i);
  return game_list;
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
