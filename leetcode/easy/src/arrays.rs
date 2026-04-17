use std::collections::HashSet;

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
