use std::collections::{HashMap, VecDeque};

pub fn create_network() {
    let mut network: HashMap<&str, VecDeque<&str>> = HashMap::new();
    let mut search_queue: VecDeque<&str> = VecDeque::new();
    let mut searched: VecDeque<&str> = VecDeque::new();

    network.insert("me", VecDeque::from(vec!["alice", "bob", "claire"]));
    network.insert("bob", VecDeque::from(vec!["anuj", "peggy"]));
    network.insert("alice", VecDeque::from(vec!["peggy"]));
    network.insert("claire", VecDeque::from(vec!["thom", "jonny"]));
    network.insert("anuj", VecDeque::new());
    network.insert("peggy", VecDeque::new());
    network.insert("thom", VecDeque::new());
    network.insert("jonny", VecDeque::new());

    if let Some(friends) = network.get_mut("me") {
        search_queue.append(friends);
    }

    while !search_queue.is_empty() {
        match search_queue.pop_front() {
            Some(person) if !searched.contains(&person) => {
                if person_is_seller(person) {
                    println!("Person {} is a mango seller!", person);
                } else {
                    if let Some(friends) = network.get_mut(person) {
                        search_queue.append(friends);
                    }
                    searched.push_back(person);
                }
            }
            _ => print!("The search queue is empty"),
        }
    }
}

fn person_is_seller(person: &str) -> bool {
    person.chars().last().unwrap() == 'm'
}
