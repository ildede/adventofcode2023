use std::str::Chars;

pub fn solve_part_a(input: &str) -> String {
    let lines = input.lines().into_iter().collect::<Vec<_>>();
    let result: u32 = lines.clone()
        .into_iter()
        .enumerate()
        .flat_map(|(index, el)| to_numbers(index, el))
        .filter(|n| {
            let mut is_ok: bool = false;
            for pos in n.pos.iter() {
                let up_line: Option<Chars> = if n.line > 0 {
                    Some(lines[n.line - 1].chars())
                } else {
                    None
                };
                let cur_line = lines[n.line].chars();
                let down_line: Option<Chars> = if n.line < (lines.len() - 1) {
                    Some(lines[n.line + 1].chars())
                } else {
                    None
                };

                let left_char_index = if pos > &0 { *pos - 1 } else { *pos };
                let cent_char_index = *pos;
                let right_char_index = if pos < &lines[0].len() { *pos + 1 } else { *pos };

                let up_left = up_line.clone().unwrap_or(cur_line.clone()).clone().nth(left_char_index);
                let up_center = up_line.clone().unwrap_or(cur_line.clone()).clone().nth(cent_char_index);
                let up_right = up_line.unwrap_or(cur_line.clone()).clone().nth(right_char_index);

                let cur_left = cur_line.clone().nth(left_char_index);
                let cur_right = cur_line.clone().nth(right_char_index);

                let down_left = down_line.clone().unwrap_or(cur_line.clone()).clone().nth(left_char_index);
                let down_center = down_line.clone().unwrap_or(cur_line.clone()).clone().nth(cent_char_index);
                let down_right = down_line.clone().unwrap_or(cur_line.clone()).clone().nth(right_char_index);

                if up_left.is_some_and(is_symbol)
                    || up_center.is_some_and(is_symbol)
                    || up_right.is_some_and(is_symbol)
                    || cur_left.is_some_and(is_symbol)
                    || cur_right.is_some_and(is_symbol)
                    || down_left.is_some_and(is_symbol)
                    || down_center.is_some_and(is_symbol)
                    || down_right.is_some_and(is_symbol)
                {
                    is_ok = true;
                }
            }
            is_ok
        })
        .map(|n| n.value)
        .sum::<u32>();

    result.to_string()
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn to_numbers(index: usize, line: &str) -> Vec<Number> {
    let mut result: Vec<Number> = Vec::new();
    let mut tmp = Number { value: 0, line: index, pos: Vec::new() };
    for (pos, e) in line.chars().collect::<Vec<_>>().iter().enumerate() {
        if e.is_ascii_digit() {
            let mut vec1 = tmp.pos.clone();
            vec1.push(pos);
            tmp = Number { value: tmp.value*10 + e.to_digit(10).unwrap_or(0), line: tmp.line, pos: vec1 }
        } else {
            if tmp.value > 0 {
                result.push(tmp.clone());
                tmp = Number { value: 0, line: index, pos: Vec::new() };
            }
        }
    }
    if tmp.value > 0 {
        result.push(tmp.clone());
    }
    result
}

#[derive(Debug, PartialEq, Clone)]
struct Number {
    value: u32,
    line: usize,
    pos: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve_part_a(
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "4361");
    }

    #[test]
    fn line_to_numbers() {
        let result = to_numbers(0, "467..114..");

        assert_eq!(result, vec![
            Number { value: 467, line: 0, pos: vec![0,1,2] },
            Number { value: 114, line: 0, pos: vec![5,6,7] }
        ])
    }

    #[test]
    fn lines_to_numbers() {
        let input =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result: Vec<Number> = input.lines().clone()
            .into_iter()
            .enumerate()
            .flat_map(|(index, el)| to_numbers(index, el))
            .collect();

        assert_eq!(result, vec![
            Number { value: 467, line: 0, pos: vec![0,1,2] },
            Number { value: 114, line: 0, pos: vec![5,6,7] },
            Number { value: 35, line: 2, pos: vec![2,3] },
            Number { value: 633, line: 2, pos: vec![6,7,8] },
            Number { value: 617, line: 4, pos: vec![0,1,2] },
            Number { value: 58, line: 5, pos: vec![7,8] },
            Number { value: 592, line: 6, pos: vec![2,3,4] },
            Number { value: 755, line: 7, pos: vec![6,7,8] },
            Number { value: 664, line: 9, pos: vec![1,2,3] },
            Number { value: 598, line: 9, pos: vec![5,6,7] }
        ])
    }

    #[test]
    fn lines_to_numbers_with_number_at_the_end() {
        let input =
"467....114
.....+..58";
        let result: Vec<Number> = input.lines().clone()
            .into_iter()
            .enumerate()
            .flat_map(|(index, el)| to_numbers(index, el))
            .collect();

        assert_eq!(result, vec![
            Number { value: 467, line: 0, pos: vec![0,1,2] },
            Number { value: 114, line: 0, pos: vec![7,8,9] },
            Number { value: 58, line: 1, pos: vec![8,9] },
        ])
    }
}
