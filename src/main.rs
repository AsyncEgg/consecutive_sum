use std::cmp::Ordering;

fn main() {
    println!("{}", _consecutive_sum_weird(9))
}

fn _consecutive_sum_for(num: usize) -> bool {
    let range: Vec<usize> = (1..=num / 3).collect();

    for n in &range {
        let windows = range.windows(*n);

        for w in windows {
            if w.iter().sum::<usize>() == num {
                println!("winner {w:?}");
                return true;
            }
        }
    }

    false
}

fn _consecutive_sum_iter(num: usize) -> bool {
    let range: Vec<usize> = (1..=num).collect();
    range
        .windows(2)
        .chain((1..=num / 4).flat_map(|n| range.windows(n + 2)))
        .any(|w| w.iter().sum::<usize>() == num)
}

fn _consecutive_sum_iter_better(num: usize) -> bool {
    let range: Vec<usize> = (1..=num).collect();

    let mut res = false;

    for n in &range {
        if !res {
            range
                .windows(*n + 1)
                .for_each(|w| res = w.iter().sum::<usize>() == num);
            break;
        }
    }
    res
}

fn _consecutive_sum_weird(num: usize) -> bool {
    let mut start = 1;
    let mut end = 1;
    let mut sum = 1;

    while start <= num / 2 {
        match sum.cmp(&num) {
            Ordering::Less => {
                end += 1;
                sum += end;
            }
            Ordering::Greater => {
                sum -= start;
                start += 1;
            }
            Ordering::Equal => return true,
        }
    }

    false
}
