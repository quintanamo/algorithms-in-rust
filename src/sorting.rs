pub fn insertion_sort(input: Vec<i32>) -> Vec<i32> {
    let mut sorted: Vec<i32> = input;

    for j in 1..sorted.len() {
        let key: i32 = sorted[j];
        let mut i: usize = j;
        while i > 0 && sorted[i - 1] > key {
            sorted[i] = sorted[i - 1];
            i = i - 1;
        }
        sorted[i] = key;
    }

    return sorted;
}

fn is_sorted(input: Vec<i32>) -> bool {
    let mut last_num = 0;
    for num in input {
        if num < last_num {
            return false;
        }
        last_num = num;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;
    use super::is_sorted;

    const INPUTS: [[i32; 5]; 5] = [[1, 2, 3, 4, 5], [5, 4, 3, 2, 1], [1, 1, 1, 2, 3], [5, 1, 4, 2, 3], [5, 1, 2, 5, 5]];

    #[test]
    fn test_is_sorted() {
        const EXPECTED_RESULTS: [bool; 5] = [true, false, true, false, false];
        for i in 0..INPUTS.len() {
            assert_eq!(is_sorted(INPUTS[i].to_vec()), EXPECTED_RESULTS[i]);
        }
    }

    #[test]
    fn test_insertion_sort() {
        for i in 0..INPUTS.len() {
            let sorted: Vec<i32> = insertion_sort(INPUTS[i].to_vec());
            assert_eq!(is_sorted(sorted), true);
        }
    }
}