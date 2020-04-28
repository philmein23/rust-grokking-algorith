mod binary_search;
mod breadth_first_search;
mod djikstras_algorithm;
mod greedy_algorithms;
mod quick_sort;
mod recursion;
mod selection_sort;

use binary_search::binary_search;
use breadth_first_search::create_network;
use djikstras_algorithm::setup_hash_tables;
use greedy_algorithms::find_station_with_most_coverage;
use quick_sort::quick_sort;
use recursion::{find_largest, find_length, sum};
use selection_sort::selection_sort;

fn main() {
    let list = vec![1, 2, 3, 4, 5, 60, 100];
    let mut list_two = vec![30, 5, 10, 2, 100];
    let list_three = vec![20, 24, 25, 30, 300, 223, 23, 24, 450];
    let result = quick_sort(&list_three);
    // let result = binary_search(&list, 0, list.len() - 1, 100);
    // let result = selection_sort(&mut list_two);
    // let result_two = sum(&mut list_three);
    // let result_two = find_length(&mut list_three);
    // let result_two = find_largest(&list_three);
    println!("{:?}", result);

    create_network();
    setup_hash_tables();
    find_station_with_most_coverage();
}
