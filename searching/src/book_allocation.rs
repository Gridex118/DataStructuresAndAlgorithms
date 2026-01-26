fn is_acceptable(pages: &[i32], students: usize, max: i32) -> bool {
    let mut page_count: i32 = 0;
    let mut student_count: usize = 1;
    for page in pages.iter() {
        if (page_count + *page) <= max {
            page_count += *page;
        } else {
            student_count += 1;
            if student_count > students {
                return false;
            }
            page_count = *page;
        }
    }
    true
}

pub fn get_min_maximum_pages(pages: &[i32], students: usize) -> i32 {
    let mut min: i32 = 0;
    let mut max: i32 = pages.iter().sum();
    while min < max {
        let mid: i32 = min + (max - min) / 2;
        if is_acceptable(pages, students, mid) {
            max = mid;
        } else {
            min = mid + 1;
        }
    }
    min + (max - min) / 2
}
