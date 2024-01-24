fn bubble_sort(tab: &mut [i32]) {
    for i in 0..tab.len()
    {
        for j in 0..tab.len() - 1 - i {
            if tab[j] > tab[j + 1] {
                tab.swap(j, j + 1);
            }
        }
    }
}

fn main() {

    let mut numbers = [4, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("After sorting: {:?}", numbers);
}
