use std::fs;
use regex::Regex;
use std::path::Path;

fn sum_of_all_muls(input_str: &str) -> i64 {
  Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
    .unwrap()
    .captures_iter(&input_str)
    .map(|cap| {
      let x: i64 = cap[1].parse().unwrap();
      let y: i64 = cap[2].parse().unwrap();
      x * y
    })
   .sum()
}

fn sum_of_active_muls(input_str: &str) -> i64 {
  let re = Regex::new(r"(do\(\)|don't\(\))").unwrap();

  let mut parts = re.split(input_str);
  let mut conds = re.find_iter(input_str);

  let (mut sum, mut active) = (0, true);

  while let Some(p) = parts.next() {
    if active {
      sum += sum_of_all_muls(p);
    }

    if let Some(c) = conds.next() {
      active = c.as_str() == "do()";
    }
  }

  sum
}

pub fn solve() {
  let input_file_path = Path::new(file!()).parent().unwrap().join("input.txt");

  let input_str = fs::read_to_string(&input_file_path).unwrap_or_else(|_| {
    eprintln!("error reading file: {:?}", input_file_path);
    std::process::exit(1);
  });

  println!("pt.1) sum of all valid muls(): {}", sum_of_all_muls(&input_str));
  println!("pt.2) sum of all valid \"active\" muls(): {}", sum_of_active_muls(&input_str));
}
