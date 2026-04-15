fn main() {
    let first_matrix = vec![
                                            vec![1, 2],
                                            vec![3, 4],
                                            vec![5, 6],
                                        ];

    let second_matrix = vec![
                                            vec![7, 8, 9,],
                                            vec! [10, 11, 12]
                                        ];
    let result = matrix_multiply(&first_matrix, &second_matrix, );
    match result {
        Ok(value) => {
            for row in &value {
                println!("{:?}", row)
            }
        }
        Err(error) => {
            println!("{:?}", error)
        }
    }
}

fn matrix_multiply(a: &[Vec<i32>], b: &[Vec<i32>]) -> Result<Vec<Vec<i32>>, String>{
    if a.len() != b[0].len() {
        return Err(String::from("matrixes can't be multiplied, sizes don't match"));
    }
    let mut result: Vec<Vec<i32>> = vec![vec![0; b[0].len()]; a.len()];
    for i in 0..a.len() {
        for k in 0..a[0].len() {
            for j in 0..b[0].len() {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    Ok(result)
}