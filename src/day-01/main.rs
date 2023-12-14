use indexmap::indexmap;

fn main() {
  let input_1 = include_str!("input.txt").split("\n").collect::<Vec<&str>>();
  let input_2 = include_str!("input.txt").split("\n").collect::<Vec<&str>>();

  solution_1(input_1);
  solution_2(input_2);
}

fn solution_1(input: Vec<&str>) {
  let mut total: i64 = 0;
  input.iter().for_each(|line| {
    let numbers: Vec<String> = line
      .chars()
      .filter(|c| c.is_numeric())
      .map(|c| c.to_digit(10).unwrap().to_string())
      .collect();

    if numbers.len() == 1 {
      let first = numbers.first().unwrap();
      let last = numbers.first().unwrap();
      total += (first.to_owned() + &*last.to_owned())
        .parse::<i64>()
        .unwrap();
    } else if numbers.len() > 1 {
      let first = numbers.first().unwrap();
      let last = numbers.last().unwrap();
      total += (first.to_owned() + &*last.to_owned())
        .parse::<i64>()
        .unwrap();
    }
  });
  println!("Total: {:?}", total);
}

fn solution_2(mut input: Vec<&str>) {
  let mut only_numbers_line = String::new();
  let mut only_numbers_vector = vec![];
  let mut words_to_numbers = indexmap! {
      "oneight" => 18,
      "twone" => 21,
      "threight" => 38,
      "fiveight" => 58,
      "eightwo" => 82,
      "eighthree" => 83,
      "one" => 1,
      "two" => 2,
      "three" => 3,
      "four" => 4,
      "five" => 5,
      "six" => 6,
      "seven" => 7,
      "eight" => 8,
      "nine" => 9,
  };

  input.iter_mut().for_each(|line| {
    only_numbers_line = line.to_string();
    words_to_numbers.iter_mut().for_each(|(key, value)| {
      if line.contains(key) {
        only_numbers_line = only_numbers_line.replace(key, &*value.to_string());
      }
    });
    only_numbers_vector.push(only_numbers_line.clone());
  });
  solution_1(
    only_numbers_vector
      .clone()
      .iter()
      .map(|x| x.as_str())
      .collect::<Vec<&str>>(),
  );
}
