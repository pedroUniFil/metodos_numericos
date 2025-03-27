fn f(x: f64) -> f64 {
    3.0 * (x + 1.0) * (x - 0.5) * (x - 1.0)
}

fn bisection_method(mut a: f64, mut b: f64, max_iter: usize) -> f64 {
    for _ in 0..max_iter {
        let c = (a + b) / 2.0;
        let fc = f(c);
        println!("Iteration: a = {:.5}, b = {:.5}, c = {:.5}, f(c) = {:.5}", a, b, c, fc);

        if f(a) * fc < 0.0 {
            b = c;
        } else {
            a = c;
        }
    }

    (a + b) / 2.0
}

fn main() {
    let a = -2.0;
    let b = 1.5;
    let max_iter = 3;  // número de iterações

    let approximate_root = bisection_method(a, b, max_iter);
    println!("Approximate root after {} iterations: {:.5}", max_iter, approximate_root);
}