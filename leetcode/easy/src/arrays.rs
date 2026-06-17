use std::collections::{HashMap, HashSet};

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let nums1: HashSet<i32> = HashSet::from_iter(nums1);
    let nums2: HashSet<i32> = HashSet::from_iter(nums2);
    nums1.intersection(&nums2)
        .cloned().collect()
}

pub fn intersection_no_uniq(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let (mut nums1, mut nums2) = (nums1, nums2);
    nums1.sort();
    nums2.sort();
    let mut intersection = Vec::new();
    let (mut p, mut q) = (0, 0);
    while p < nums1.len() && q < nums2.len() {
        if nums1[p] < nums2[q] {
            p += 1;
        } else if nums1[p] > nums2[q] {
            q += 1;
        } else {
            intersection.push(nums1[p]);
            p += 1;
            q += 1;
        }
    }
    intersection
}

pub fn third_max(nums: Vec<i32>) -> i32 {
    let (mut x, mut y, mut z) = (None, None, None);
    for num in nums {
        if [x, y, z].contains(&Some(num)) {
            continue;
        }
        if x.is_none() || num > x.unwrap() {
            (z, y, x) = (y, x, Some(num));
        } else if y.is_none() || num > y.unwrap() {
            (z, y) = (y, Some(num));
        } else if z.is_none() || num > z.unwrap() {
            z = Some(num);
        }
    }
    z.unwrap_or(x.unwrap())
}

pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut answer = Vec::new();
    for (i, &x) in nums.iter().enumerate() {
        answer.insert(index[i] as usize, x);
    }
    answer
}

pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let (m, n) = (mat.len(), mat[0].len());
    let (r, c) = (r as usize, c as usize);
    if m * n != r * c {
        mat
    } else {
        let mut result: Vec<Vec<i32>> = vec![vec![0; c]; r];
        // Matrix 1D index = total_cols * i_row + i_col;
        // i_row = 1D index / total_cols; i_col = 1D index % total_cols
        for i in 0..(r * c) {
            result[i / c][i % c] = mat[i / n][i % n];
        }
        result
    }
}

pub fn _count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    let i_key = match rule_key.as_str() {
        "type"  => 0,
        "color" => 1,
        "name"  => 2,
        &_ => panic!("Invalid rule key")
    };
    items.iter()
        .filter(|v| v[i_key] == rule_value)
        .count() as i32
}

pub fn smaller_number_than_current(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .map(|x| nums.iter().fold(0, |acc, next| {
            if next < x {
                acc + 1
            } else {
                acc
            }
        }))
        .collect()
}

pub fn _num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        map.entry(x).or_default().push(i as i32);
    };
    map.values().fold(0, |acc, v| {
        let n = v.len() as i32;
        acc + (n * (n - 1)) / 2
    })
}

pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut left = 0;
    let mut right = letters.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if letters[mid] > target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if left < letters.len() {
        letters[left]
    } else {
        letters[0]
    }
}

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut result = Vec::with_capacity(2 * n);
    for i in 0..n {
        result.push(nums[i]);
        result.push(nums[i + n]);
    }
    result
}

fn sr_binary_search(nums: &[i32], target: i32, first: bool) -> Option<i32> {
    let mut left = 0;
    let mut right = nums.len().checked_sub(1)?;
    let mut i = -1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            i = mid as i32;
            if first {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else if nums[mid] > target {
            right = mid.checked_sub(1)?;
        } else {
            left = mid + 1;
        }
    }
    Some(i)
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    vec![sr_binary_search(&nums, target, true).unwrap_or(-1),
         sr_binary_search(&nums, target, false).unwrap_or(-1)]
}

fn _peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < arr[mid + 1] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left as i32
}

fn _majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut answer = nums[0];
    for &num in nums.iter() {
        if count == 0 {
            answer = num;
        }
        count += if num == answer { 1 } else { -1 };
    }
    answer
}

pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for x in nums {
        *counts.entry(x).or_insert(0) += 1;
    }
    let mut max_length = 0;
    for (x, count) in &counts {
        let next = x + 1;
        if let Some(count_next) = counts.get(&next) {
            max_length = max_length.max(count + count_next);
        }
    }
    max_length
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len() as i32;
    assert!(!matrix.is_empty());
    let n = matrix[0].len() as i32;
    let mut result = Vec::new();
    // Start at (0, 0) with a velocity of (1, 0) till the right border
    // You make a 90° turn if the next cell is already visited, or if coordinates go out of bounds
    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (1, 0);
    let mut visited = vec![false; (m * n) as usize];
    for _ in 0..(m * n) {
        result.push(matrix[y as usize][x as usize]);
        visited[(y * n + x) as usize] = true;
        let (nx, ny) = (x + dx, y + dy);
        let next_out_of_bounds = nx < 0 || nx >= n || ny < 0 || ny >= m;
        if next_out_of_bounds || visited[(ny * n + nx) as usize] {
            (dx, dy) = (-dy, dx);
        }
        x += dx;
        y += dy;
    }
    result
}
