/*

    INSERTION SORT

*/
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

/*

    MERGE SORT

*/
pub fn merge_sort(input: Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    let mut sorted: Vec<i32> = input;
    if left < right {
        let middle: usize = left + (right - left) / 2;
        sorted = merge_sort(sorted, left, middle);
        sorted = merge_sort(sorted, middle + 1, right);
        sorted = merge(sorted, left, middle, right);
    }
    return sorted;
}

fn merge(input: Vec<i32>, left: usize, middle: usize, right: usize) -> Vec<i32> {
    let mut sorted: Vec<i32> = input;
    let n1: usize = middle - left + 1;
    let n2: usize = right - middle;
    let mut left_vec: Vec<i32> = vec![-1; n1];
    let mut right_vec: Vec<i32> = vec![-1; n2];
    for i in 0..n1 {
        left_vec[i] = sorted[left + i];
    }
    for j in 0..n2 {
        right_vec[j] = sorted[middle + j + 1];
    }
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = left;
    while i < n1 && j < n2 {
        if left_vec[i] <= right_vec[j] {
            sorted[k] = left_vec[i];
            i = i + 1;
        } else {
            sorted[k] = right_vec[j];
            j = j + 1;
        }
        k = k + 1;
    }
    while i < n1 {
        sorted[k] = left_vec[i];
        i = i + 1;
        k = k + 1;
    }
    while j < n2 {
        sorted[k] = right_vec[j];
        j = j + 1;
        k = k + 1;
    }
    return sorted;
}

/*

    UTILITY FUNCTIONS

*/
// function only used to validate if inputs are sorted
#[allow(dead_code)]
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
    use super::merge_sort;
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

    #[test]
    fn test_merge_sort() {
        for i in 0..INPUTS.len() {
            let sorted: Vec<i32> = merge_sort(INPUTS[i].to_vec(), 0, INPUTS[i].len() - 1);
            println!("{sorted:?}");
            assert_eq!(is_sorted(sorted), true);
        }
    }
}