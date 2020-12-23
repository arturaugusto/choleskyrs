fn main() {
    let matrix:Vec<Vec<f64>>;
    // based on https://github.com/hville/cholesky
    //var tri = cho([[4, 12, -16], [12, 37, -43], [-16, -43, 98]])
    //console.log(tri[0]) // [2]
    //console.log(tri[1]) // [6, 1]
    //console.log(tri[2]) // [-8, 5, 3]
    
    // input matrix
    matrix = vec![vec![4.0, 12.0, -16.0], vec![12.0, 37.0, -43.0], vec![-16.0, -43.0, 98.0]];
    
    let mut res = vec![vec![0.0f64]; matrix.len()];

    res[0][0] = matrix[0][0].sqrt();

    for i in 1..matrix.len() {
        res[i] = vec![0.0; i+1];
        for j in 0..i {
            res[i][j] = delta(matrix[i][j], &res, i, j) / res[j][j];
        }
    res[i][i] = (delta(matrix[i][i], &res, i, i)).sqrt();
    }
    println!("{:?}", res);
}

fn delta(aij:f64, res:&Vec<Vec<f64>>, i:usize, j:usize) -> f64 {
    let mut sum = aij;
    let mut k = 0;
    while k < j {
      if res[i][k] != 0f64 {
          sum = sum - res[i][k] * res[j][k]
      }
      k = k + 1;
    }
    return sum
}
