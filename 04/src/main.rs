use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let connections = input.trim().lines().map(|l| l.split_once(" <-> ").unwrap()).collect::<Vec<_>>();
    let locations = connections.iter().fold(HashSet::<&str>::new(), |mut set, (l, r)| {
        set.insert(l);
        set.insert(r);
        set
    });
    println!("part1: {}", locations.len());

    let cons: HashMap<&str, HashSet<&str>> = connections.iter().fold(HashMap::new(), |mut map, (l, r)| {
        map.entry(l).or_insert_with(HashSet::new).insert(r);
        map.entry(r).or_insert_with(HashSet::new).insert(l);
        map
    });
    // println!("cons: {:?}", &cons);
    let mut results: HashSet<&str> = HashSet::new();
    travel("STT", 0, &cons, &mut results);
    println!("part2: {}", results.len());

    let p3 = locations.iter().map(|loc| shortest_dist("STT", loc, 0, &cons)).sum::<i32>();
    println!("part3: {}", p3);
}

fn shortest_dist(start: &str, end: &str, dist: i32, cons: &HashMap<&str, HashSet<&str>>) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back((start, dist));
    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some((node, dist)) = queue.pop_front() {
        if node == end {
            return dist;
        }
        for neighbor in cons.get(node).unwrap_or(&HashSet::new()) {
            if !visited.contains(neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, dist + 1));
            }
        }
    }
    -1
}

fn travel<'a>(cur: &'a str, time: i32, cons: &HashMap<&'a str, HashSet<&'a str>>, results: &mut HashSet<&'a str>) {
    if time > 3 {
        return;
    }
    results.insert(cur);
    for next in cons.get(cur).unwrap_or(&HashSet::new()) {
        travel(next, time + 1, cons, results);
    }
}
