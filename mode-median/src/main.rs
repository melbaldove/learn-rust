use std::collections::HashMap;

fn main() {
    let int_list = [1, 3, 1, 5, 6, 3, 3, 4, 9, 8, 4];

    let (median, mode) = calculate_mode_median(&int_list);

    println!("Median: {0}, Mode: {1}", median, mode);
}

fn calculate_mode_median(int_vec: &[i32]) -> (i32, i32) {
    let mut sorted = int_vec.to_vec();
    sorted.sort();
    println!("The sorted list is {:?}", sorted);
    let median = sorted[(sorted.len() - 1) / 2];
    let mut map = HashMap::new();

    for x in sorted {
        let count = map.entry(x).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut most_frequent_count = 0;
    for (x, count) in map {
        if count > most_frequent_count {
            most_frequent_count = count;
            mode = x
        }
    }

    return (median, mode);
}
