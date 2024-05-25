use std::fs;

const SYMBOLS: [char; 12] = ['~', '@', '#', '$', '%', '^', '&', '*', '+', '=', '/', '-'];

fn main() {
    println!("Running part 1:");
    let input = fs::read_to_string("src/input_pt1.txt").unwrap();
    let answer = part_one(&input);
    println!("ANSWER= {}", answer);
}

fn has_symbol_near(x_pos: (usize, usize), pos_y: usize, schematic: &Vec<Vec<char>>) -> bool {
    let left = 
        if x_pos.0 <= 0 {
            x_pos.0
        } else {
            x_pos.0 - 1
        };
    let right = 
        if x_pos.1 >= schematic[pos_y].len() - 1 {
            x_pos.1
        } else {
            x_pos.1 + 1
        };
    dbg!(&schematic[pos_y][left..=right]);

    //check above
    if pos_y > 0 {
        for character in &schematic[pos_y - 1][left..=right] {
            if SYMBOLS.contains(character) {
                return true;
            }
        }
    }
    //check sides
    if SYMBOLS.contains(&schematic[pos_y][left]) || SYMBOLS.contains(&schematic[pos_y][right]) {
        return true;
    }
    //check below
    if pos_y + 1 < schematic.len() {
        for character in &schematic[pos_y + 1][left..=right] {
            if SYMBOLS.contains(character) {
                return true;
            }
        }
    }

    false
}

fn part_one(input: &str) -> u32 {

    let mut parts_sum = 0;
    let schema: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    dbg!(schema.len());

    for (y_index, row) in schema.iter().enumerate() {
        dbg!(&y_index);
        let mut str_num = String::new();
        let mut x_positions: Vec<usize> = Vec::new();
        let iterator = row.iter().enumerate();
        for (i, c) in iterator {
            if c.is_numeric() {
                str_num = format!("{str_num}{}",c);
                x_positions.push(i);
                if i == row.len() - 1 {
                    let part_number: u32 = str_num.parse()
                        .expect("the part number was not in fact a number");
                    let x_min = x_positions.first().unwrap().clone();
                    let x_max = x_positions.last().unwrap().clone();
                    if has_symbol_near((x_min, x_max), y_index, &schema) {
                        println!("{} IS A PART", str_num);
                        parts_sum += part_number;
                    }

                }
            } else {
                if !str_num.is_empty() {
                    let part_number: u32 = str_num.parse()
                        .expect("the part number was not in fact a number");
                    let x_min = x_positions.first().unwrap().clone();
                    let x_max = x_positions.last().unwrap().clone();
                    if has_symbol_near((x_min, x_max), y_index, &schema) {
                        println!("{} IS A PART", str_num);
                        parts_sum += part_number;
                    }
                }
                str_num.clear();
                x_positions.clear();
            }
        }
    }


    parts_sum
}

#[cfg(test)]
mod part_1_test {
    use super::*;
    use std::fs;

    #[test]
    fn test_pt1_mini(){
        let input = fs::read_to_string("src/input_pt1_sm.txt").expect("file not found");
        assert_eq!(part_one(&input), 4361);
    }

    #[test]
    fn symbol_near_ends(){
        let test_schematic = vec![
            vec!['0', '0', '*', '0'],
            vec!['1', '1', '1', '1'],
            vec!['2', '2', '2', 'X'], // false
            vec!['3', '*', '3', '3'],
            vec!['4', '4', 'X', 'X'], // true
            vec!['5', '5', '5', '5'], 
            vec!['6', '6', 'X', 'X'], //true
            vec!['7', '/', '7', 'X'], // false
        ];
            assert_eq!(has_symbol_near((3, 3), 2, &test_schematic), false);
            assert_eq!(has_symbol_near((2, 3), 4, &test_schematic), true);
            assert_eq!(has_symbol_near((2, 3), 6, &test_schematic), true);
            assert_eq!(has_symbol_near((3, 3), 7, &test_schematic), false);
    }

    #[test]
    fn that_one_failure() {
        let test_schematic: Vec<Vec<char>> = r###"
...........303.........605..326.....108.......%..56..842=.....*....387...*..478-..272=....*..........#........778.....299=..@...722.........
......484=..=.....753..........$......+.....449...............559.......144.............695.........675..................................253
...-...............*.........*................................................................987.......200......*......445.......124...*...
....825............603....544.634........................432...=875...738...............731..*.............&..488.363..*.....663.........876"###
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        assert_eq!(test_schematic.len(), 4);
        for row in &test_schematic {
            assert_eq!(row.len(), 140);
        }
        assert_eq!(&test_schematic[1][137..140], ['2','5','3']);
        assert_eq!(&test_schematic[3][137..140], ['8','7','6']);
        assert_eq!(has_symbol_near((137, 139), 1, &test_schematic), true);
        assert_eq!(has_symbol_near((137, 139), 3, &test_schematic), true);
    }

    #[test]
    fn symbol_near_line_zero() {
        let test_schematic = vec![
            vec!['.', '3', '*', '.'], // true
            vec!['1', '.', '.', '.'],
            vec!['.', '.', '.', '9'], // false
            vec!['.', '*', '.', '.'],
            vec!['.', '.', '2', '4'], // true
        ];
            assert_eq!(has_symbol_near((1, 1), 0, &test_schematic), true);
            assert_eq!(has_symbol_near((0, 0), 1, &test_schematic), false);
            assert_eq!(has_symbol_near((3, 3), 2, &test_schematic), false);
            assert_eq!(has_symbol_near((2, 3), 4, &test_schematic), true);
    }

    #[test]
    fn symbol_list_matches() {
        let input = fs::read_to_string("src/input_pt1.txt").unwrap();
        let file_symbols: Vec<char> = input.chars().filter(|c| { c.is_numeric()==false && *c != '.' && *c != '\n'}).collect();
        for sym in file_symbols {
            assert_eq!(SYMBOLS.contains(&sym), true);
        }
    }
}

