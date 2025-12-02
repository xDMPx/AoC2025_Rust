pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let id_ranges = puzzle_input
        .split(',')
        .map(|x| x.trim().split_once('-').unwrap());

    let mut sum = 0;
    for (start_range, end_range) in id_ranges {
        let sequence_len = if start_range.len() % 2 == 0 {
            Some(start_range.len() / 2)
        } else if end_range.len() % 2 == 0 {
            Some(end_range.len() / 2)
        } else {
            None
        };
        if let Some(sequence_len) = sequence_len {
            let start = if (start_range.len() % 2) != 0 {
                let mut x = 1;
                for _ in 0..(sequence_len - 1) {
                    x *= 10;
                }
                x
            } else {
                start_range.parse::<u64>().unwrap() / u64::pow(10, sequence_len as u32)
            };
            let end = if (end_range.len() % 2) != 0 {
                let mut x = 0;
                for i in 0..sequence_len {
                    x += 9 * (u64::pow(10, i as u32));
                }
                x
            } else {
                end_range.parse::<u64>().unwrap() / u64::pow(10, sequence_len as u32)
            };

            for i in start..=end {
                let mut sequence = i.to_string();
                sequence.extend(sequence.clone().chars());
                let id: u64 = sequence.parse().unwrap();
                if id >= start_range.parse().unwrap() && id <= end_range.parse().unwrap() {
                    sum += id;
                }
            }
        }
    }

    sum.try_into().unwrap()
}
