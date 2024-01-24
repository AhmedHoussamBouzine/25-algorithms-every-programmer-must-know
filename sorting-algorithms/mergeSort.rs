fn merge_sort(tab: &mut [i32]) {
    if tab.len() <= 1 {
        return;
    }
    let mid = tab.len() / 2;
    let mut left = tab[..mid].to_vec();
    let mut right = tab[mid..].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(tab, &left, &right);
}

fn merge(tab: &mut [i32], left: &[i32], right: &[i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            tab[k] = left[i];
            i += 1;
        } else {
            tab[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        tab[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        tab[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut numbers = [4, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", numbers);
    merge_sort(&mut numbers);
    println!("After sorting: {:?}", numbers);
}
