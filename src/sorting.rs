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

    #[test]
    fn test_is_sorted() {
        let mut input: Vec<i32> = [5,2,3,1,4].to_vec();
        assert_eq!(is_sorted(input), false);
        input = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(is_sorted(input), true);
        input = [1, 1, 2, 2, 3].to_vec();
        assert_eq!(is_sorted(input), true);
    }

    #[test]
    fn test_insertion_sort() {
        let mut input: Vec<i32> = [5,2,3,1,4].to_vec();
        let mut output: Vec<i32> = insertion_sort(input);
        assert_eq!(is_sorted(output), true);
        input = [1, 1, 2, 1, 1].to_vec();
        output = insertion_sort(input);
        assert_eq!(is_sorted(output), true);
    }
}