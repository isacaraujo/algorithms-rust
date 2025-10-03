use std::cmp::Ordering;

/// It compares two given words and returns:
/// - Ordering::Greater if a > b
/// - Ordering::Less if a < b
/// - Ordering::Equal if a == b
///
/// # Example
///
/// ```
/// assert_eq!(cmp("abc", "abc"), Ordering::Equal);
/// assert_eq!(cmp("a", "abc"), Ordering::Less);
/// assert_eq!(cmp("abc", "a"), Ordering::Greater);
/// ```
fn cmp(one: &str, other: &str) -> Ordering {
    let a: Vec<char> = one.chars().collect();
    let b: Vec<char> = other.chars().collect();

    for i in 0..a.len() {
        let b_ = match b.get(i) {
            Some(val) => val,
            None => return Ordering::Greater,
        };

        let result = a[i].cmp(b_);

        match result {
            Ordering::Equal => continue,
            _ => return result,
        };
    }

    // At this point 'one' was fully iterated
    // resting only to check if the length of
    // 'other' is greater than.
    // If so, it returns a < b
    // Ex:
    //  abc < abcd
    if b.len() > a.len() {
        return Ordering::Less;
    }

    // Otherwise, we assume both are Equal
    Ordering::Equal
}


fn main() {
    let tests = vec![
        ("abc", "a", Ordering::Greater),
        ("abc", "aabc", Ordering::Greater),
        ("def", "abc", Ordering::Greater),
        ("aaa", "bbb", Ordering::Less),
        ("abc", "abc", Ordering::Equal),
    ];

    for (one, other, expected) in tests {
        let result = cmp(one, other);

        let status = if result == expected { "PASSED" } else { "FAILED" };

        println!("({}) {} <> {} = {:?}", status, one, other, result);
    }
}
