use crate::SolutionResult::{Infinite, NoSolution, Unique};

enum SolutionResult {
    Unique(Vec<f64>),
    Infinite,
    NoSolution,
}
fn gaussian_elimination(matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut matrix = matrix.clone();
    let rows = matrix.len();
    let cols = matrix[0].len();
    for pivot in 0..rows {
        for row in pivot + 1..rows {
            let multiplier = matrix[row][pivot] / matrix[pivot][pivot];
            for col in 0..cols {
                matrix[row][col] = matrix[row][col] - (multiplier * matrix[pivot][col]);
            }
        }
    }
    matrix
}

fn back_substitution(matrix: Vec<Vec<f64>>) -> SolutionResult {
    let rows = matrix.len();
    let cols = matrix[0].len() - 1;
    let mut result = vec![0.0; rows];

    for row in (0..rows).rev() {
        let mut variable = matrix[row][cols];
        for j in (row + 1)..rows {
            variable -= matrix[row][j] * result[j];
        }
        if matrix[row][row].abs() < 1e-10 {
            if variable.abs() < 1e-10 {
                return Infinite
            } else {
                return NoSolution
            }
        }
        result[row] = variable / matrix[row][row];

    }
    Unique(result)
}

fn main() {
    let matrix = vec![
        vec![2.0,  1.0, -1.0,  8.0],
        vec![-3.0, -1.0,  2.0, -11.0],
        vec![-2.0,  1.0,  2.0, -3.0],
    ];

    let result = gaussian_elimination(matrix.clone());
    for row in &result {
        println!("{:?}", row);
    }

    match back_substitution(result) {
        SolutionResult::Unique(solution) => println!("{:?}", solution),
        SolutionResult::Infinite => println!("Infinite solutions"),
        SolutionResult::NoSolution => println!("No solution"),
    }
}
