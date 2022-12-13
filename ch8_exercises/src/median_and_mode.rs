use std::collections::HashMap;

fn div_ceil(a: &i32, b: &i32) -> i32 {
    (a + b - 1) / b
}

pub fn median(v: &Vec<i32>) -> Option<f64> {
    let mut v_clone = v.to_vec();
    let v_size = v.len() as i32;
    v_clone.sort();

    if v_size == 0 {
        return None;
    } else if v_size == 1 {
        return Some(v_clone[0] as f64);
    } else if v_size % 2 != 0 {
        let median_index = div_ceil(&v_size, &2);
        return Some(v_clone[(median_index - 1) as usize] as f64);
    } else {
        let median_index_high = div_ceil(&v_size, &2);
        let median_index_low = (v_size / 2) - 1;
        let median_sum = v_clone[median_index_high as usize] + v_clone[median_index_low as usize];
        return Some(median_sum as f64 / 2.0);
    }
}

pub fn mode(v: &Vec<i32>) -> Option<Vec<i32>> {
    if v.len() == 0 {
        return None;
    }
    let mut v_mode = Vec::new();
    let mut number_count_map = HashMap::new();
    for x in v {
        let count = number_count_map.entry(*x).or_insert(0);
        *count += 1;
    }

    let mut mode_count = 0;
    for (_, val) in &number_count_map {
        if *val > mode_count {
            mode_count = *val;
        }
    }

    for (key, val) in &number_count_map {
        if *val == mode_count {
            v_mode.push(*key);
        }
    }

    Some(v_mode)
}
