fn main() {
    let a = [
        [5.0, 2.0, 2.0],
        [1.0, 3.0, 1.0],
        [0.0, 6.0, 8.0],
    ];
    
    let b = [3.0, -2.0, -6.0];
    
    let mut x = [0.6, -0.7, -0.75];
    
    let iterations = 3;
    
    for _ in 0..iterations {
        let new_x1 = (b[0] - a[0][1] * x[1] - a[0][2] * x[2]) / a[0][0];
        let new_x2 = (b[1] - a[1][0] * x[0] - a[1][2] * x[2]) / a[1][1];
        let new_x3 = (b[2] - a[2][0] * x[0] - a[2][1] * x[1]) / a[2][2];
        
        x[0] = new_x1;
        x[1] = new_x2;
        x[2] = new_x3;
        
        println!("Iteração: x1 = {:.6}, x2 = {:.6}, x3 = {:.6}", x[0], x[1], x[2]);
    }
}