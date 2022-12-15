use std::collections::HashMap;

fn div_ceil(a: &i32, b: &i32) -> i32 {
    (a + b - 1) / b
}

pub fn median(v: &Vec<i32>) -> Option<f64> {
    let mut v_clone = v.to_vec();
    let v_size = v.len() as i32;
    v_clone.sort();

    if v.is_empty() {
        None
    } else if v_size == 1 {
        Some(v_clone[0] as f64)
    } else if v_size % 2 != 0 {
        let median_index = div_ceil(&v_size, &2);
        Some(v_clone[(median_index - 1) as usize] as f64)
    } else {
        let median_index_high = div_ceil(&v_size, &2);
        let median_index_low = (v_size / 2) - 1;
        let median_sum = v_clone[median_index_high as usize] + v_clone[median_index_low as usize];
        Some(median_sum as f64 / 2.0)
    }
}

pub fn mode(v: &Vec<i32>) -> Option<Vec<i32>> {
    if v.is_empty() {
        return None;
    }

    let mut number_count_map = HashMap::new();
    for x in v {
        let count = number_count_map.entry(*x).or_insert(0);
        *count += 1;
    }

    let highest_occurance = *number_count_map.values().max().unwrap();
    let v_mode = number_count_map
        .iter()
        .filter(|(_, &v)| v == highest_occurance)
        .map(|(&k, _)| k)
        .collect::<Vec<i32>>();

    Some(v_mode)
}
