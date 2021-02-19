use std::collections::{HashMap, HashSet};

use algorithms::*;

fn main() {
    let mut states_needed = set(&["mt", "wa", "or", "id", "nv", "ut", "ca", "az"]);

    let mut stations = HashMap::new();

    stations.insert("kone", set(&["id", "nv", "ut"]));
    stations.insert("ktwo", set(&["wa", "id", "mt"]));
    stations.insert("kthree", set(&["or", "nv", "ca"]));
    stations.insert("kfour", set(&["nv", "ut"]));
    stations.insert("kfive", set(&["ca", "az"]));

    let mut final_stations = HashSet::new();

    while states_needed.len() > 0 {
        let mut best_station = None;
        let mut states_covered = HashSet::new();
        for (station, states_for_station) in &stations {
            let covered = &states_needed & states_for_station;
            if covered.len() > states_covered.len() {
                best_station = Some(station);
                states_covered = covered;
            }
        }

        if let Some(station) = best_station {
            states_needed = &states_needed - &states_covered;
            final_stations.insert(station);
        } else {
            panic!("unable to find station for states {:?}", states_needed);
        }
    }

    println!("{:?}", final_stations);
}
