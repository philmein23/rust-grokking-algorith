use std::collections::{HashMap, HashSet};

pub fn find_station_with_most_coverage() {
    let mut stations: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut station_one = HashSet::new();
    let mut station_two = HashSet::new();
    let mut station_three = HashSet::new();
    let mut station_four = HashSet::new();
    let mut station_five = HashSet::new();

    for name in vec!["id", "nv", "ut"].into_iter() {
        station_one.insert(name);
    }
    for name in vec!["wa", "id", "mt"].into_iter() {
        station_two.insert(name);
    }
    for name in vec!["or", "nv", "ca"].into_iter() {
        station_three.insert(name);
    }
    for name in vec!["nv", "ut"].into_iter() {
        station_four.insert(name);
    }
    for name in vec!["ca", "az"].into_iter() {
        station_five.insert(name);
    }

    for (name, associated_station) in vec![
        ("kone", station_one),
        ("ktwo", station_two),
        ("kthree", station_three),
        ("kfour", station_four),
        ("kfive", station_five),
    ]
    .into_iter()
    {
        stations.insert(name, associated_station);
    }

    let final_stations = calculate(&stations);
    println!("FINAL: {:?}", final_stations);
}

fn calculate<'a>(stations: &HashMap<&'a str, HashSet<&str>>) -> HashSet<&'a str> {
    let mut final_stations = HashSet::new();
    let mut states_needed = HashSet::new();
    let mut best_station = "";

    for name in vec!["mt", "wa", "or", "id", "nv", "ut", "ca", "az"].into_iter() {
        states_needed.insert(name);
    }

    while !states_needed.is_empty() {
        let mut states_covered = HashSet::new();

        for (station, states_for_station) in stations {
            let covered = states_needed
                .intersection(states_for_station)
                .copied()
                .collect::<HashSet<_>>();

            if covered.len() > states_covered.len() {
                best_station = *station;
                states_covered = covered;
            }
        }

        states_needed = states_needed
            .difference(&states_covered)
            .copied()
            .collect::<HashSet<_>>();
        final_stations.insert(best_station);
    }

    final_stations
}
