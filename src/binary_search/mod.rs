// Big O is O(log(n))
pub fn binary_search(list: &Vec<u32>, low: usize, high: usize, item: u32) -> Option<usize> {
    // let mut low = 0;
    // let mut high = list.len() - 1;

    if list.len() == 0 {
        return None;
    }

    if list.len() == 1 {
        let guess = list.get(0).unwrap().to_owned();
        if guess == item {
            return Some(0);
        } else {
            return None;
        }
    }

    if low <= high {
        let mid = (low + high) / 2;
        let guess = list.get(mid).unwrap().to_owned();

        if guess == item {
            return Some(mid);
        }

        if guess > item {
            let val = binary_search(list, low, mid - 1, item).unwrap();
            return Some(val);
        } else {
            let val = binary_search(list, mid + 1, high, item).unwrap();
            return Some(val);
        }
    } else {
        return None;
    }

    // while low <= high {
    //     let mid = (low + high) / 2;
    //     let guess = list.get(mid).unwrap();

    //     if guess == item {
    //         return Some(mid);
    //     }

    //     if guess > item {
    //         high = mid - 1;
    //     } else {
    //         low = mid + 1;
    //     }
    // }
    // return None;
}
