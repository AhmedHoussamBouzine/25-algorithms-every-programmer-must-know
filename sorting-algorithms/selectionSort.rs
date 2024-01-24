fn selection_sort(tab: &mut [i32]) {
    for i in 0..tab.len() - 1 {
        let mut min_index = i;
        for j in i + 1..tab.len() {
            if tab[j] < tab[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            tab.swap(i, min_index);
        }
    }
}

fn main() {

    let mut numbers = [4, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", numbers);
    selection_sort(&mut numbers);
    println!("After sorting: {:?}", numbers);
}
