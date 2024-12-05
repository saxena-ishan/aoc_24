use std::fs;
use std::path::Path;

fn is_report_safe(report: &[i64]) -> bool {
  if report.len() < 2 {
    return true;
  }

  let is_asc = report[1] > report[0];

  report.windows(2).all(|w| {
    let delta = (w[1] - w[0]).abs();
    ((w[1] > w[0]) == is_asc) && (1..=3).contains(&delta)
  })
}

fn is_report_safe_with_dampener(report: &[i64]) -> bool {
  (0..report.len()).any(|i| {
    is_report_safe(
      &report
        .iter()
        .enumerate()
        .filter_map(|(j, &n)| (i != j).then_some(n))
        .collect::<Vec<i64>>()
    )
  })
}

pub fn solve() {
  let input_file_path = Path::new(file!()).parent().unwrap().join("input.txt");

  let input_str = fs::read_to_string(&input_file_path).unwrap_or_else(|_| {
    eprintln!("error reading file: {:?}", input_file_path);
    std::process::exit(1);
  });

  let reports: Vec<Vec<i64>> = input_str
    .lines()
    .map(|line| {
      line
        .split_whitespace()
        .map(|n| n.parse().expect("Invalid number"))
        .collect()
    })
    .collect();

  let (safe_reps, unsafe_reps): (Vec<&Vec<i64>>, Vec<&Vec<i64>>) = reports.iter().partition(
    |r| is_report_safe(r)
  );

  println!("pt.1) safe count: {}", safe_reps.len());

  let safe_unsafe_reps = unsafe_reps.iter().filter(
    |r| is_report_safe_with_dampener(r)
  );

  println!("pt.2) safe count with dampener: {}", (safe_reps.len() + safe_unsafe_reps.count()));
}
