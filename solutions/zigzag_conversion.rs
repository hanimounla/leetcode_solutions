/*
The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

P   A   H   N
A P L S I I G
Y   I   R

And then read line by line: "PAHNAPLSIIGYIR"
*/
pub fn convert(s: String, num_rows: i32) -> String {
    let chars: Vec<char> = s.chars().collect();

    if num_rows == 1 || num_rows as usize >= chars.len() {
        return s;
    }

    let mut rows: Vec<Vec<char>> = vec![Vec::new(); num_rows as usize];
    let mut idx: i32 = 0;
    let mut d: i32 = 1;

    for &c in &chars {
        rows[idx as usize].push(c);

        if idx == 0 {
            d = 1;
        } else if idx == (num_rows as i32 - 1) {
            d = -1;
        }

        idx += d;
    }

    rows.into_iter().flatten().collect()
}

pub fn convert_old(s: String, num_rows: i32) -> String {
    let mut result: String = Default::default();

    // initialize a 2d array where the y axis is known
    // and the x axis is based on the word
    let mut matrix: Vec<Vec<String>> = Vec::with_capacity(num_rows as usize);
    let chars: Vec<char> = s.chars().collect();
    let chars_length = chars.len() - 1;
    let mut row_index = 0;

    for i in 0..chars_length {
        let character_to_add = chars[i].to_string();
        if matrix.len() == 0 {
            matrix.push(vec![character_to_add.clone()]);
        }
        if row_index == (num_rows - 1) as usize {
            row_index -= 1;
        } else {
            row_index += 1;
        }
        if matrix[row_index - 1].len() == 0 {
            matrix.push(vec![character_to_add]);
        } else {
            matrix[row_index].push(character_to_add);
        }
    }

    // read strings row by row
    for row in matrix {
        for col in row {
            result += &col;
        }
    }

    result
}
