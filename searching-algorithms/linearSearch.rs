fn linear_search(tab: &[i32], target: i32) -> Option<usize> {
    let mut index = 0;
    while index < tab.len() {
        if tab[index] == target {
            return Some(index);
        }
        index += 1;
    }
    None
}

fn main() {
    let mut numbers = [4, 2, 9, 1, 5, 6];
    let value = 2;
    if let Some(index) = linear_search(&mut numbers, value) {
        println!("Found {} at index {}", value, index);
    } else {
        println!("{} not found in the list", value);
    }
}