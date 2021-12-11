fn partition<T: PartialOrd>(a: &mut [T], left: usize, right: usize, pivot: usize) -> usize {
    a.swap(pivot, right);
    let mut store_index = left;
    for i in left..right {
        if a[i] < a[right] {
            a.swap(store_index, i);
            store_index += 1;
        }
    }
    a.swap(right, store_index);
    store_index
}

fn pivot_index(left: usize, right: usize) -> usize {
    left + (right - left) / 2
}

fn select<T: PartialOrd>(a: &mut [T], mut left: usize, mut right: usize, n: usize) {
    loop {
        if left == right {
            break;
        }
        let mut pivot = pivot_index(left, right);
        pivot = partition(a, left, right, pivot);
        match n.cmp(&pivot) {
            std::cmp::Ordering::Less => right = pivot - 1,
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Greater => left = pivot + 1,
        };
    }
}

// Rearranges the elements of 'a' such that the element at index 'n' is
// the same as it would be if the array were sorted, smaller elements are
// to the left of it and larger elements are to its right.
pub fn nth_element<T: PartialOrd + Copy>(a: &mut [T], n: usize) -> T {
    select(a, 0, a.len() - 1, n);
    a[n]
}
