pub mod sortera {
    use std::fmt::Debug;

    // O(n^2)
    pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
        let mut sorted = true;
        for i in 0..v.len() - 1 {
            for j in 0..v.len() - i - 1 {
                if v[j] > v[j + 1] {
                    v.swap(j, j + 1);
                    sorted = false;
                }
            }
            if sorted {
                return;
            }
        }
    }

    // O(nlog(n))
    pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
        // sort the left half using merge sort
        // sort the right half using merge sort
        // bring the sorted halves together
        if v.len() <= 1 {
            return v;
        }

        let mut res = Vec::with_capacity(v.len());
        let r = v.split_off(v.len() / 2);
        let a = merge_sort(v);
        let b = merge_sort(r);

        // bring them together
        let mut a_it = a.into_iter();
        let mut b_it = b.into_iter();
        let mut a_peek = a_it.next();
        let mut b_peek = b_it.next();

        loop {
            match a_peek {
                Some(ref a_val) => match b_peek {
                    Some(ref b_val) => {
                        if b_val < a_val {
                            res.push(b_peek.take().unwrap());
                            b_peek = b_it.next();
                        } else {
                            res.push(a_peek.take().unwrap());
                            a_peek = a_it.next();
                        }
                    }
                    None => {
                        res.push(a_peek.take().unwrap());
                        res.extend(a_it);
                        return res;
                    }
                },
                None => {
                    if let Some(b_val) = b_peek {
                        res.push(b_val)
                    }
                    res.extend(b_it);
                    return res;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sortera::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 2, 3, 6, 1, 5];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![4, 2, 3, 6, 1, 5];
        let r = merge_sort(v);
        assert_eq!(r, vec![1, 2, 3, 4, 5, 6]);
    }
}
