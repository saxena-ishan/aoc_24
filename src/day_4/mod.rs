use std::fs;
use std::path::Path;

fn count_xmas(matrix: &Vec<Vec<char>>) -> i64 {
  let mut result: i64 = 0;

  let (row_c, col_c) = (matrix.len(), matrix[0].len());

  let moves = [
    (-1, 0),  // up
    (1, 0),   // down
    (0, 1),   // right
    (0, -1),  // left
    (-1, -1), // up-left
    (1, 1),   // down-right
    (-1, 1),  // up-right
    (1, -1),  // down-left
  ];

  let is_xmas = |i: i64, j: i64, dx: i64, dy: i64| -> bool {
    matrix[i as usize][j as usize] == 'X'
    && (((i + (3 * dx)) >= 0) && ((i + (3 * dx)) < (row_c as i64)))
    && (((j + (3 * dy)) >= 0) && ((j + (3 * dy)) < (col_c as i64)))
    && (matrix[(i + dx) as usize][(j + dy) as usize] == 'M')
    && (matrix[(i + (2 * dx)) as usize][(j + (2 * dy)) as usize] == 'A')
    && (matrix[(i + (3 * dx)) as usize][(j + (3 * dy)) as usize] == 'S')
  };

  for i in 0..row_c {
    for j in 0..col_c {
      for &(dx, dy) in &moves {
        if is_xmas(i as i64, j as i64, dx, dy) {
          result += 1;
        }
      }
    }
  }

  result
}


fn count_x_mas(matrix: &Vec<Vec<char>>) -> i64 {
  let mut result: i64 = 0;

  let (row_c, col_c) = (matrix.len(), matrix[0].len());

  let is_x_mas = |i, j| -> bool {
    if !(
      matrix[i as usize][j as usize] == 'A'
      && (((i - 1) >= 0) && ((j - 1) >= 0) && (((i + 1) as usize) < row_c) && (((j + 1) as usize) < col_c)) 
    ) {
      return false;
    };

    let seqs = [
      ('M', 'M', 'S', 'S'),
      ('S', 'M', 'M', 'S'),
      ('S', 'S', 'M', 'M'),
      ('M', 'S', 'S', 'M'),
    ];

    for &(a, b, c, d) in &seqs {
      if 
        matrix[(i - 1) as usize][(j - 1) as usize] == a
        && matrix[(i - 1) as usize][(j + 1) as usize] == b
        && matrix[(i + 1) as usize][(j + 1) as usize] == c
        && matrix[(i + 1) as usize][(j - 1) as usize] == d
      {
        return true;
      }
    }

    return false;
  };

  for i in 0..row_c {
    for j in 0..col_c {
      if is_x_mas(i as i64, j as i64) {
        result += 1;
      }
    }
  }

  result
}

pub fn solve() {
  let input_file_path = Path::new(file!()).parent().unwrap().join("input.txt");

  let input_str = fs::read_to_string(&input_file_path).unwrap_or_else(|_| {
    eprintln!("error reading file: {:?}", input_file_path);
    std::process::exit(1);
  });

  let matrix: Vec<Vec<char>> = input_str.lines().map(
    |line| line.trim().chars().collect()
  ).collect();

  println!("pt.1) count of XMAS: {}", count_xmas(&matrix));

  println!("pt.2) count of X-MAS: {}", count_x_mas(&matrix));
}
