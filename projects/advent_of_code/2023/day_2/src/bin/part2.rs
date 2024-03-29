mod part1;

use std::fs;

use crate::part1::day_2::*;

fn main() {
    println!("part 2");

    let input = fs::read_to_string("src/input_pt1.txt").expect("could not find file or something");
    let answer = part_2(&input);
    println!("ANSWER PT_2: {}", answer);

}

fn part_2(input: &str) -> u32 {
    let answer = input.lines()
        .map(|game| {
            let (game_str, rounds_str) = game.split_once(":").unwrap();
            let game_id = game_str.split_once(" ").unwrap().1
                .parse::<u32>().unwrap();

            let mut color_map: (CubePull, CubePull, CubePull) = (
                CubePull{ color: Color::Red, count: 0},
                CubePull{ color: Color::Green, count: 0},
                CubePull{ color: Color::Blue, count: 0}
                );

            let rounds: Vec<CubePull> = rounds_str.split(";")
                .map(|round| {
                    round.trim().split(",")
                        .map(|pull|CubePull::from_string(pull))
                        .collect::<Vec<CubePull>>()
                }).flatten()
            .collect();

            for pull in rounds {
                match pull.color {
                    Color::Red => {
                        if color_map.0.count < pull.count { color_map.0.count = pull.count};
                    },
                    Color::Green => {
                        if color_map.1.count < pull.count {color_map.1.count = pull.count};
                    },
                    Color::Blue => {
                        if color_map.2.count < pull.count {color_map.2.count = pull.count};
                    },
                };
            }

            (game_id, color_map)
        })
    .collect::<Vec<(u32, (CubePull, CubePull, CubePull))>>();
    dbg!(answer);
    100_000
}

#[cfg(test)]
mod part_2_tests {
    use std::fs;
    use super::*;

    #[test]
    fn small_answer_works() {
        // uses same file as part 1
        let input_small = fs::read_to_string("src/input_pt1_sm.txt")
            .expect("problem with file");
        assert_eq!(part_2(&input_small), 2286);
    }
}
