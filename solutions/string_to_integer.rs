pub fn my_atoi(s: String) -> i32 {
    let mut result: i32 = 0;
    let mut temp_string = "".to_string();
    let mut found_number = false;
    let mut found_sign = false;

    for c in s.chars() {
        let char_str = c.to_string();
        let stop = c.is_alphabetic() || char_str == ".";
        let sign = char_str == "-" || char_str == "+";
        let space = char_str == " ";

        if c.is_numeric() {
            found_number = true;
            temp_string += &char_str;
            continue;
        }

        if found_number {
            if space || stop || sign {
                break;
            }
        }

        if space && found_sign {
            return result;
        }

        if space {
            continue;
        }

        if !found_number {
            if stop {
                return result;
            }
            if sign {
                found_sign = true;
                temp_string += &char_str;
            }
        }
    }
    if temp_string.len() == 0 {
        return result;
    }

    let min: i128 = i32::MIN as i128;
    let max: i128 = i32::MAX as i128;

    let temp_result = match temp_string.parse::<i128>() {
        Ok(num) => num,
        Err(_) => {
            for c in temp_string.chars() {
                if !c.is_numeric() {
                    return 0;
                }
            }
            if temp_string.starts_with("-") {
                return min as i32;
            }
            return max as i32;
        } // or return 0, as per the behavior for invalid input
    };

    let temp_result_i128 = match temp_string.parse::<i128>() {
        Ok(num) => num,
        Err(_) => return result,
    };
    if temp_result_i128 < min {
        result = min as i32;
    } else if temp_result_i128 > max {
        result = max as i32;
    } else {
        result = temp_result as i32;
    }

    return result;
}
