use gridit::{Grid, PositionsEnumerator};
use itertools::Itertools;

fn main() {
  let max_x = 140;
  let max_y = 140;

  let input = include_str!("input.txt")
    .split("\n")
    .collect::<Vec<&str>>()
    .iter()
    .map(|line| line.to_string().chars().collect_vec())
    .collect_vec();

  println!(
    "Total: {:?}",
    solution_1(
      max_x,
      max_y,
      &Grid::from(input.iter().flatten().collect_vec(), max_x, max_y)
    )
  );

  println!(
    "Total: {:?}",
    solution_2(
      max_x,
      max_y,
      &Grid::from(input.iter().flatten().collect_vec(), max_x, max_y)
    )
  );
}

fn solution_2(max_x: usize, max_y: usize, grid: &Grid<&char>) -> usize {
  let mut total = 0;
  for row in 0..max_x {
    for column in 0..max_y {
      let value = grid.get((column, row)).unwrap();
      if !(value.is_numeric() || **value == '.') {
        let gears = grid
          .neighbors((column, row))
          .grid_positions()
          .map(|(pos, value)| {
            return if value.is_numeric() {
              find_full_neighbor_number(&grid, pos.x, pos.y)
            } else {
              String::from("")
            };
          })
          .sorted()
          .dedup()
          .filter_map(|s| s.parse::<usize>().ok())
          .collect_vec();
        if gears.len() == 2 {
          total += gears.first().unwrap() * gears.last().unwrap();
        } else {
          continue;
        }
      }
    }
  }
  return total;
}

fn solution_1(max_x: usize, max_y: usize, grid: &Grid<&char>) -> usize {
  let mut total = 0;
  for row in 0..max_x {
    for column in 0..max_y {
      let value = grid.get((column, row)).unwrap();
      if !(value.is_numeric() || **value == '.') {
        total += grid
          .neighbors((column, row))
          .grid_positions()
          .map(|(pos, value)| {
            return if value.is_numeric() {
              find_full_neighbor_number(&grid, pos.x, pos.y)
            } else {
              String::from("")
            };
          })
          .sorted()
          .dedup()
          .filter_map(|s| s.parse::<usize>().ok())
          .sum::<usize>();
      }
    }
  }
  return total;
}

fn find_full_neighbor_number(grid: &Grid<&char>, x: usize, y: usize) -> String {
  let mut i = 0;
  let mut number = String::from("");

  if grid.get((x, y)).unwrap().is_numeric()
    && grid.get((x + 1, y)).unwrap().is_numeric()
    && grid.get((x - 1, y)).unwrap().is_numeric()
  {
    return String::from("");
  }
  if grid.get((x + 1, y)).unwrap().is_numeric() {
    while grid
      .get((x + i, y))
      .or_else(|| Some(&&' '))
      .unwrap()
      .is_numeric()
    {
      number += &grid.get((x + i, y)).unwrap().to_string();
      i += 1;
    }
    return number;
  }
  if grid.get((x - 1, y)).unwrap().is_numeric() {
    while grid
      .get((x - i, y))
      .or_else(|| Some(&&' '))
      .unwrap()
      .is_numeric()
    {
      number += &grid.get((x - i, y)).unwrap().to_string();
      i += 1;
    }
    return number
      .chars()
      .rev()
      .flat_map(|c| c.to_digit(10))
      .collect_vec()
      .iter()
      .map(|c| c.to_string())
      .join("");
  }
  if !grid.get((x + 1, y)).unwrap().is_numeric() && !grid.get((x - 1, y)).unwrap().is_numeric() {
    return grid.get((x, y)).unwrap().to_string();
  }
  return String::from("");
}
