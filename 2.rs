fn fib(n: usize) -> usize {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    // Using Dijkstra's formula for integer Fibonacci calculation:
    //
    // fib(n) = (2 * fib(n/2 - 1) + fib(n/2)) * fib(n/2)  } if n is even
    // fib(n) = fib(n+1 / 2)^2 + fib((n+1 / 2) - 1)^2     } if n is odd
    if n % 2 == 0 {
        let fib_n_half = fib(n / 2);
        let fib_n_half_less_1 = fib((n / 2) - 1);
        return ((2 * fib_n_half_less_1) + fib_n_half) * fib_n_half;
    } else {
        let fib_n_half = fib((n + 1) / 2);
        let fib_n_half_less_1 = fib(((n + 1) / 2) - 1);
        return (fib_n_half * fib_n_half) + (fib_n_half_less_1 * fib_n_half_less_1);
    }
}

fn main() {
    // Every 3rd Fibonacci number starting at the first is even.
    let sum = (0..).map(|n| n * 3)
                   .map(|n| fib(n))
                   .take_while(|&n| n < 4 * 1000 * 1000) // 4 million
                   .fold(0, |sum, n| sum + n);
    println!("{}", sum);
}
