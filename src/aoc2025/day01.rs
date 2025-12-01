#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: i64,
}

pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let rotations = puzzle_input.lines().map(|line| {
        let direction = match line {
            line if line.starts_with("R") => Direction::Right,
            line if line.starts_with("L") => Direction::Left,
            _ => unreachable!(),
        };

        let distance = line.chars().skip(1).collect::<String>().parse().unwrap();

        Rotation {
            direction,
            distance,
        }
    });

    let mut dial_value: i64 = 50;
    let mut zero_count = 0;

    for roration in rotations {
        match roration.direction {
            Direction::Left => dial_value -= roration.distance % 100,
            Direction::Right => dial_value += roration.distance % 100,
        }
        if dial_value < 0 {
            dial_value += 100;
        } else if dial_value > 99 {
            dial_value -= 100;
        }

        if dial_value == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let rotations = puzzle_input.lines().map(|line| {
        let direction = match line {
            line if line.starts_with("R") => Direction::Right,
            line if line.starts_with("L") => Direction::Left,
            _ => unreachable!(),
        };

        let distance = line.chars().skip(1).collect::<String>().parse().unwrap();

        Rotation {
            direction,
            distance,
        }
    });

    let mut dial_value: i64 = 50;
    let mut zero_count = 0;

    for roration in rotations {
        for _ in 0..roration.distance {
            match roration.direction {
                Direction::Left => dial_value -= 1,
                Direction::Right => dial_value += 1,
            }
            if dial_value == 100 {
                dial_value = 0;
            } else if dial_value == -1 {
                dial_value = 99;
            }
            if dial_value == 0 {
                zero_count += 1;
            }
        }
    }

    zero_count
}
