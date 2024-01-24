fn insertion_sort(tab: &mut [i32]) {
    for i in 1..tab.len() {
        let mut j = i;
        while j > 0 && tab[j - 1] > tab[j] {
            tab.swap(j - 1, j);
            j -= 1;
        }
    }
}
fn main() {

    let mut numbers = [4, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", numbers);
    insertion_sort(&mut numbers);
    println!("After sorting: {:?}", numbers);
}
