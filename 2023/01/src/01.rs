aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .fold(0, |sum, line| {
            let mut digits = line.chars().filter(|c| c.is_ascii_digit());
            let first = digits.next().unwrap().to_digit(10).unwrap();
            let last = match digits.last() {
                Some(d) => d.to_digit(10).unwrap(),
                None => first,
            };
            sum + first * 10 + last
        })
        .to_string()
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
