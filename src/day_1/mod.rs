use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub fn solve() {
  let input_file_path = Path::new(file!()).parent().unwrap().join("input.txt");

  let input_str = fs::read_to_string(&input_file_path).unwrap_or_else(|_| {
    eprintln!("error reading file: {:?}", input_file_path);
    std::process::exit(1);
  });

  // parse
  let (mut arr1, mut arr2): (Vec<i64>, Vec<i64>) = (Vec::new(), Vec::new());
  let mut arr2_freqs: HashMap<i64, i64> = HashMap::new();

  for line in input_str.lines() {
    let mut parts = line.split_whitespace();

    if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
      let p_a: i64 = a.parse().expect("invalid number");
      let p_b: i64 = b.parse().expect("invalid number");

      arr1.push(p_a);
      arr2.push(p_b);

      *arr2_freqs.entry(p_b).or_insert(0) += 1;
    }
    else {
      eprintln!("invalid line: {}", line);
    }
  };

  // sort
  arr1.sort();
  arr2.sort();

  // calc distance
  let dist: i64 = arr1
    .iter()
    .zip(arr2.iter())
    .map(|(a, b)| (a - b).abs())
    .sum();

  println!("pt.1) dist: {}", dist);

  // calc sim score
  let sim_score: i64 = arr1
    .iter()
    .map(|e| *e * (*arr2_freqs.get(e).unwrap_or(&0) as i64))
    .sum();

  println!("pt.2) sim score: {}", sim_score);
}
