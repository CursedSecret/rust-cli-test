pub mod kaprekar {
    pub fn kaprekar(number: i32) -> i32 {
        if number == 6174 {
            return 0;
        } else if !is_valid_number(number) {
            return -1;
        } else {
            let mut number_array: [i32; 4] = convert_to_array(number);
            number_array.sort();
            let large = convert_to_number(number_array);
            number_array.reverse();
            let small = convert_to_number(number_array);

            return kaprekar(large - small) + 1;
        }
    }

    fn is_valid_number(number: i32) -> bool {
        return number < 10000 && has_non_duplicate_digit(number);
    }

    fn has_non_duplicate_digit(number: i32) -> bool {
        let mut result = false;
        let array = convert_to_array(number);
        for inner_index in 1..4 {
            for outer_index in 1..4 {
                if array[inner_index] != array[outer_index] && inner_index != outer_index {
                    result = true;
                }
            }
        }
        result
    }

    fn convert_to_array(number: i32) -> [i32; 4] {
        let mut result = [0; 4];
        let mut index = 0;
        let mut number = number;
        while number > 0 {
            result[index] = number % 10;
            number = number / 10;
            index += 1;
        }
        result
    }

    fn convert_to_number(array: [i32; 4]) -> i32 {
        let mut result = 0;
        let mut factor = 1;
        for digit in array {
            result += digit * factor;
            factor *= 10;
        }
        result
    }
}
