use std::fs;

fn main() {
    let input_file = fs::read_to_string("src/input.txt").unwrap();

    let numbers: Vec<_> = input_file
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .collect();

    let unsafe_number = find_unsafe_numbers(&numbers, 100);

    match unsafe_number {
        Some(num) => println!("EXIT: Found unsafe number: {:?}", num),
        None => println!("all numbers are safe."),
    }
}

fn find_unsafe_numbers(numbers: &[u32], check_range: usize) -> Option<usize> {
    for i in check_range..numbers.len() {
        let previous_numbers = &numbers[(i - check_range)..i];
        let current_number = numbers[i];

        let is_number_safe = previous_numbers.iter().any(|x| {
            previous_numbers
                .iter()
                .any(|y| x + y == current_number && x != y)
        });

        if !is_number_safe {
            return Some(current_number.try_into().unwrap());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_five_are_safe() {
        let test_input = vec![35, 20, 15, 25, 47];
        let find_unsafe = find_unsafe_numbers(&test_input, 5);
        assert_eq!(find_unsafe, None);
    }

    #[test]
    fn test_for_unsafe_number() {
        let test_input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let find_unsafe = find_unsafe_numbers(&test_input, 5);
        assert_eq!(find_unsafe, Some(127));
    }
}

//    let first_hundred = &split_input[0..100];
//    println!("It's first a hundred. It's safe to mine: {:?}", first_hundred);

//    let unsafe_numbers = &split_input[100..split_input.len()];
//    println!("unsafe numbers: {:?}", unsafe_numbers);

//    for i in 0..unsafe_numbers.len() {

//     let current_number = unsafe_numbers[i];
//     println!("current number is {}", current_number);

//     let previous_numbers = &unsafe_numbers[(i-100)..i];

// for j in 0..100 {

//     for k in j + 1..100 {

//         if previous_numbers[j] + previous_numbers[k] == current_number {
//             println!("It's safe to continue mining; The current number:{:?} is a sum of: {:?} and {:?}", current_number, previous_numbers[j], previous_numbers[k]);
//         } else {
//             break;
//         }
//     }
// }

//  }
