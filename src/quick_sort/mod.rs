pub fn quick_sort(list: &Vec<u32>) -> Vec<u32> {
    if list.len() < 2 {
        return list.to_owned();
    } else {
        let pivot = list.get(0).unwrap();
        let mut less = vec![];
        let mut greater = vec![];
        for val in list {
            if val < pivot {
                less.push(val.to_owned());
            }

            if val > pivot {
                greater.push(val.to_owned());
            }
        }

        quick_sort(&less)
            .into_iter()
            .chain(vec![pivot.to_owned()].into_iter())
            .chain(quick_sort(&greater).into_iter())
            .collect::<Vec<u32>>()
    }
}
