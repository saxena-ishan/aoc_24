use std::fs;
use std::path::Path;

fn is_report_safe(report: &[i64]) -> bool {
  let mut is_asc = false;

  let mut last = report[0];

  for i in 1..report.len() {
    let _is_asc = report[i] > last;

    if i == 1 {
      is_asc = _is_asc;
    }
    else if is_asc != _is_asc {
      return false;
    };

    let diff = (report[i] - last).abs();

    if (diff < 1) || (diff > 3) {
      return false;
    }

    last = report[i];
  }

  true
}

fn is_report_safe_with_dampener(report: &[i64]) -> bool {
  for i in 0..report.len() {
    let r_without_i = [&report[..i], &report[i + 1..]].concat();

    if is_report_safe(&r_without_i) {
      return true;
    }
  }

  false
}

pub fn solve() {
  let input_file_path = Path::new(file!()).parent().unwrap().join("input.txt");

  let input_str = fs::read_to_string(&input_file_path).unwrap_or_else(|_| {
    eprintln!("error reading file: {:?}", input_file_path);
    std::process::exit(1);
  });

  let reports: Vec<Vec<i64>> = input_str.lines().map(
    |line| line.split_whitespace().map(
      |n| n.parse().expect("invalid number")
    ).collect()
  ).collect();

  let (safe_reps, unsafe_reps): (Vec<&Vec<i64>>, Vec<&Vec<i64>>) = reports.iter().partition(
    |r| is_report_safe(r)
  );

  println!("pt.1) safe count: {}", safe_reps.len());

  let safe_unsafe_reps = unsafe_reps.iter().filter(
    |r| is_report_safe_with_dampener(r)
  );

  println!("pt.2) safe count with dampener: {}", (safe_reps.len() + safe_unsafe_reps.count()));
}
