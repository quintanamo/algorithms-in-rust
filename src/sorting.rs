pub fn insertion_sort(input: Vec<i32>) -> Vec<i32> {
    let mut sorted: Vec<i32> = input;

    for j in 1..sorted.len() as i32 {
        let key: i32 = sorted[j as usize];
        let mut i: i32 = j - 1;
        while i >= 0 && sorted[i as usize] > key {
            sorted[(i + 1) as usize] = sorted[i as usize];
            i = i - 1;
        }
        sorted[(i + 1) as usize] = key;
    }

    return sorted;
}