// Big O is O(n)
fn find_smallest_value(list: &Vec<u32>) -> usize {
    let mut smallest_idx = 0 as usize;
    let mut smallest_val = list.get(0).unwrap();

    for (idx, val) in list.iter().enumerate() {
        if val < smallest_val {
            smallest_val = val;
            smallest_idx = idx;
        }
    }

    return smallest_idx;
}

// O(n^2) time
pub fn selection_sort(list: &mut Vec<u32>) -> Vec<u32> {
    let mut sorted_list = vec![];
    for _ in list.clone() {
        let smallest_idx = find_smallest_value(&list);
        let smallest_val = list.remove(smallest_idx);
        sorted_list.push(smallest_val);
    }

    return sorted_list;
}
