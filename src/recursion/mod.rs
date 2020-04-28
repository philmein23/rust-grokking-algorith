pub fn sum(list: &mut Vec<u32>) -> u32 {
    if list.len() == 0 {
        return 0;
    }

    if list.len() == 1 {
        return list.remove(0);
    }

    let first = list.remove(0);

    return first + sum(list);
}

pub fn find_length(list: &mut Vec<u32>) -> u32 {
    let mut count = 0;

    if list.is_empty() {
        return 0;
    }

    list.remove(0);

    count += 1;
    count + find_length(list)
}

pub fn find_largest(list: &Vec<u32>) -> u32 {
    // this method is memory inefficient and also has O(n!)
    // let mut largest = list.remove(0);

    // if list.len() == 2 {
    //     let next = list.remove(0);
    //     if largest < next {
    //         return next;
    //     } else {
    //         return largest;
    //     }
    // };

    // let next = find_largest(list);

    // if largest < next {
    //     largest = next;
    // }

    // largest

    // more memory efficient and also O(n)
    let mut largest = list.get(0).unwrap();

    for val in list {
        if val > largest {
            largest = val;
        }
    }

    largest.to_owned()
}
