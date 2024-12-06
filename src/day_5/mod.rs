use std::fs;
use std::path::Path;
use std::collections::{HashMap, HashSet};

fn get_mid(update: &[i64]) -> i64 {
  let mid = update.len() / 2;
  update[mid - ((update.len() % 2 == 0) as usize)]
}

fn is_valid_update(update: &[i64], rules: &HashMap<i64, HashSet<i64>>) -> bool {
  update.iter().enumerate().all(|(i, &n)| {
    i == 0 || !rules.get(&n).map_or(false, |s| {
      update[..i].iter().any(|e| s.contains(e))
    })
  })
}

fn fix_update(update: &mut Vec<i64>, rules: &HashMap<i64, HashSet<i64>>) {
  update.sort_by(|a, b| {
    match (
      rules.get(a).map_or(false, |s| s.contains(b)),
      rules.get(b).map_or(false, |s| s.contains(a)),
    ) {
      (true, false) => std::cmp::Ordering::Less,
      (false, true) => std::cmp::Ordering::Greater,
      _ => std::cmp::Ordering::Equal,
    }
  });
}

pub fn solve() {
  let input_file_path = Path::new(file!()).parent().unwrap().join("input.txt");

  let input_str = fs::read_to_string(&input_file_path).unwrap_or_else(|_| {
    eprintln!("error reading file: {:?}", input_file_path);
    std::process::exit(1);
  });

  let (rules_str, updates_str) = input_str
    .split_once("\n\n")
    .expect("invalid input");

  // rules
  let rules: HashMap<i64, HashSet<i64>> = rules_str.lines().fold(
    HashMap::new(),
    |mut acc, line| {
      let (key, value): (i64, i64) = line
        .split_once("|")
        .map(|(k, v)| (
          k.trim().parse().expect("invalid key"),
          v.trim().parse().expect("invalid value"),
        ))
        .expect("invalid rule");

      acc
        .entry(key)
        .or_insert_with(HashSet::new)
        .insert(value);

      acc
    }
  );

  // updates
  let mut updates: Vec<Vec<i64>> = updates_str
    .lines()
    .map(|line|
      line
        .split(",")
        .map(|n| n.trim().parse().expect("invalid number"))
        .collect()
    )
    .collect();

  // sum
  let (sum_1, sum_2): (i64, i64) = updates.iter_mut().fold(
    (0, 0),
    |(s1, s2), u| {
      if is_valid_update(u, &rules) {
        ((s1 + get_mid(u)), s2)
      }
      else {
        fix_update(u, &rules);
        (s1, (s2 + get_mid(u)))
      }
    }
  );

  println!("pt.1) sum for valids: {}", sum_1);
  println!("pt.2) sum for fixed invalids: {}", sum_2);
}
