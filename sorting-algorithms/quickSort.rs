fn quick_sort(tab: &mut [i32]) {
    if tab.len() <= 1 {
        return;
    }
    let pivot_index = partition(tab);
    quick_sort(&mut tab[..pivot_index]);
    quick_sort(&mut tab[pivot_index + 1..]);
}

fn partition(tab: &mut [i32]) -> usize {
    let pivot_index = tab.len() / 2;
    tab.swap(pivot_index, tab.len() - 1);

    let mut i = 0;
    for j in 0..tab.len() - 1 {
        if tab[j] <= tab[tab.len() - 1] {
            tab.swap(i, j);
            i += 1;
        }
    }

    tab.swap(i, tab.len() - 1);
    i
}

fn main() {
    let mut numbers = [4, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", numbers);
    quick_sort(&mut numbers);
    println!("After sorting: {:?}", numbers);
}
