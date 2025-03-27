fn gaussian_elimination(mut matrix: Vec<Vec<f64>>, mut results: Vec<f64>) -> Vec<f64> {
    let n = matrix.len();

    for i in 0..n {
        let mut max_row = i;
        for k in i + 1..n {
            if matrix[k][i].abs() > matrix[max_row][i].abs() {
                max_row = k;
            }
        }

        matrix.swap(i, max_row);
        results.swap(i, max_row);

        for k in i + 1..n {
            let f = matrix[k][i] / matrix[i][i];
            results[k] -= f * results[i];
            for j in i..n {
                matrix[k][j] -= f * matrix[i][j];
            }
        }
    }

    let mut solution = vec![0.0; n];
    for i in (0..n).rev() {
        solution[i] = results[i];
        for j in i + 1..n {
            solution[i] -= matrix[i][j] * solution[j];
        }
        solution[i] /= matrix[i][i];
    }

    solution
}

fn main() {
    let matrix = vec![
        vec![2.0, -3.0, 1.0],
        vec![4.0, -6.0, -1.0],
        vec![1.0, 2.0, 1.0],
    ];

    let results = vec![-5.0, -7.0, 4.0];

    let solution = gaussian_elimination(matrix, results);

    println!("Solução: {:?}", solution);
}