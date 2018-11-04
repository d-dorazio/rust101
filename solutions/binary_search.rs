pub fn binary_search(v: &[u32], el: u32) -> Option<usize> {
    use std::cmp::Ordering;

    if v.is_empty() {
        return None;
    }

    let mid = v.len() / 2;

    match el.cmp(&v[mid]) {
        Ordering::Equal => Some(mid),
        Ordering::Less => binary_search(&v[..mid], el),
        Ordering::Greater => binary_search(&v[mid + 1..], el).map(|i| i + mid + 1),
    }
}
