fn has_decrement(elem: char, nelem: char) -> bool {
    (elem == 'I' && (nelem == 'V' || nelem == 'X'))
        || (elem == 'X' && (nelem == 'L' || nelem == 'C'))
        || (elem == 'C' && (nelem == 'D' || nelem == 'M'))
}

fn convert(roman: char) -> i32 {
    match roman {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
		'M' => 1000,
        _ => 0
    }
}

pub fn roman_to_int(roman: &str) -> i32 {
    let mut result: i32 = 0;
    let roman: Vec<char> = roman.chars().collect();
    let mut riter = roman.iter().peekable();
    while let Some(&elem) = riter.next() {
        if let Some(&&nelem) = riter.peek() && has_decrement(elem, nelem) {
            result += convert(nelem) - convert(elem);
            riter.next();
        } else {
            result += convert(elem);
        }
    }
    result
}

pub fn hamming_distance(mut x: i32, mut y: i32) -> i32 {
    let mut distance = 0;
    while x > 0 || y > 0 {
        distance += (x ^ y) & 0b1;
        x >>= 1;
        y >>= 1;
    }
    distance
}
