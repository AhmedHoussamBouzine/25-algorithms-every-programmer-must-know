fn binary_search(tab: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = tab.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;

        if tab[mid] == target {
            return Some(mid);
        } else if tab[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

fn main() {
    let mut numbers = [1, 2, 4, 7, 8, 11];
    let value = 4;
    if let Some(index) = binary_search(&mut numbers, value) {
        println!("Found {} at index {}", value, index);
    } else {
        println!("{} not found in the list", value);
    }
}