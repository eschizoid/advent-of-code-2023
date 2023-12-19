use itertools::Itertools;
use nom::bytes::complete::{take_until, take_while};
use nom::character::complete as cc;
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{bytes::complete::tag, Finish, IResult};

fn main() {
  let games_1 = all_consuming(parse_all_cards)(include_str!("input.txt"))
    .finish()
    .unwrap()
    .1;

  println!("Total: {:?}", games_1);
}

pub fn parse_all_cards(i: &str) -> IResult<&str, Vec<Card>> {
  let game_list = separated_list1(cc::newline, parse_game)(i);
  return game_list;
}

fn parse_game(i: &str) -> IResult<&str, Card> {
  // Sample input:
  // Card   1: 29 21 67 44  6 13 68 15 60 79 | 75 44 60 30 10 68 40 70 36 79  3 13 64 15  4 46 21 22 67 47 73 86 29 53  6

  let (i, (_, game_id)) = tuple((tag("Card "), take_until(":")))(i)?;
  let (i, (_, games)) = tuple((
    tag(": "),
    separated_list1(tag("|"), take_while(|c| c != '|' && c != '\n')),
  ))(i)?;

  Ok((
    i,
    Card {
      card_id: game_id.to_string().trim().parse::<i32>().unwrap(),
      winning_numbers: games
        .first()
        .iter()
        .flat_map(|s| s.split_whitespace())
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec(),
      numbers_you_have: games
        .last()
        .iter()
        .flat_map(|s| s.split_whitespace())
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec(),
    },
  ))
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Card {
  pub card_id: i32,
  pub winning_numbers: Vec<i32>,
  pub numbers_you_have: Vec<i32>,
}
