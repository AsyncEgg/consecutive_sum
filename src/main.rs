fn main() {
    println!("9: {}", consecutive_sum(9));
    println!("10: {}", consecutive_sum(10));
    println!("64: {}", consecutive_sum(64))
}

fn consecutive_sum(num: usize) -> bool {
    let range: Vec<usize> = (1..=num).collect();
    range
        .windows(2)
        .chain((1..=num / 4).flat_map(|n| range.windows(n + 2)))
        .any(|w| w.iter().sum::<usize>() == num)
}
