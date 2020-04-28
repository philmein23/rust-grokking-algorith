use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn setup_hash_tables() {
    let mut graph: HashMap<&str, Vec<(&str, u32)>> = HashMap::new();
    let mut costs: HashMap<&str, u32> = HashMap::new();
    let mut parents: HashMap<&str, &str> = HashMap::new();

    graph.insert("Start", vec![("A", 6), ("B", 2)]);
    graph.insert("A", vec![("Finish", 1)]);
    graph.insert("B", vec![("A", 3), ("Finish", 5)]);
    graph.insert("Finish", vec![]);

    costs.insert("A", 6);
    costs.insert("B", 2);
    costs.insert("Finish", 1000000); // trying to represent infinity

    parents.insert("A", "Start");
    parents.insert("B", "Start");
    parents.insert("Finish", "");

    let shortest = calculate_cheapest_path(&graph);

    println!("SHORTEST: {:?}", shortest);
}

fn calculate_cheapest_path<'a>(
    graph: &HashMap<&'a str, Vec<(&'a str, u32)>>,
) -> HashMap<&'a str, u32> {
    let mut distances: HashMap<&str, u32> = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert("Start", 0);
    to_visit.push(("Start", 0));

    while let Some((node, distance)) = to_visit.pop() {
        if !visited.insert(node) {
            // already visited
            continue;
        }
        if let Some(neighbors) = graph.get(node) {
            for (n, cost) in neighbors {
                let new_cost = distance + cost;
                let is_cheaper = distances.get(n).map_or(true, |&current| current > new_cost);
                if is_cheaper {
                    distances.insert(*n, new_cost);
                    to_visit.push((*n, new_cost));
                }
            }
        }
    }

    distances
}

fn find_lowest_node_cost<'a>(costs: HashMap<&'a str, u32>, processed: Vec<&str>) -> &'a str {
    let mut lowest_cost = 10000000 as u32; // initialize to infinity representation
    let mut lowest_cost_node = "";

    for (node, cost) in costs {
        if cost < lowest_cost && !processed.contains(&node) {
            lowest_cost = cost;
            lowest_cost_node = node;
        }
    }

    lowest_cost_node
}
