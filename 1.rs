fn arithmetic_sum(a1: usize, an: usize, n: usize) -> usize {
    return (n * (a1 + an)) / 2
}

fn main() {
    let three_sum = arithmetic_sum(3, 3 * (999 / 3), 999 / 3);
    let five_sum = arithmetic_sum(5, 5 * (999 / 5), 999 / 5);
    let fifteen_sum = arithmetic_sum(15, 15 * (999 / 15), 999 / 15);

    // Remove the fifteen_sum once since it's counted twice.
    println!("{}", three_sum + five_sum - fifteen_sum);
}
