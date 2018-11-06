pub fn sort(mut v: Vec<u8>) -> Vec<u8> {
    sort_ref(&mut v);
    v
}

pub fn sort_ref(v: &mut Vec<u8>) -> &mut Vec<u8> {
    let mut n = v.len();

    while n > 0 {
        let mut newn = 0;
        for i in 1..n {
            if v[i] < v[i - 1] {
                v.swap(i - 1, i);
                newn = i;
            }
        }
        n = newn;
    }

    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(sort(vec![]), vec![]);
        assert_eq!(sort(vec![1, 2, 3, 4]), vec![1, 2, 3, 4]);
        assert_eq!(sort(vec![4, 3, 2, 1]), vec![1, 2, 3, 4]);
        assert_eq!(sort(vec![3, 1, 2, 5, 7]), vec![1, 2, 3, 5, 7]);
    }

    #[test]
    fn test_sort_ref() {
        let mut v = vec![8, 42, 12];

        assert_eq!(sort_ref(&mut v), &vec![8, 12, 42]);
        assert_eq!(v, vec![8, 12, 42]);
    }
}
