use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    length: u8,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cost: u32,
}

#[derive(Debug, Clone)]
struct Node {
    edges: Vec<Edge>,
}

pub fn run(flights: Vec<(usize, usize, u32)>, limits: (usize, usize, u8)) {
    //flights: (from, to, cost)
    //limits: (from, to, maximum number of flights willing to take)

    println!("{:?}", dijkstra(flights, limits));
}

fn dijkstra(flights: Vec<(usize, usize, u32)>, limits: (usize, usize, u8)) 
-> Option<u32> {
    let count = 1 + std::cmp::max(
        flights.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0,
        flights.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1
        );
    
    let mut cities = vec![Node{edges: Vec::new()}; count];

    for flight in flights {
        cities[flight.0].edges.push(Edge{to: flight.1, cost: flight.2});
    }

    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    let mut dist: Vec<HashMap<u8, u32>> = vec![HashMap::new(); count];
    for city in &mut dist {
        for i in 0..=limits.2 { city.insert(i, u32::MAX); }
    }
    dist[limits.0].insert(0, 0);

    heap.push(State{cost: 0, length: 0, position: limits.0});

    while let Some(State{cost, length, position}) = heap.pop() {
        if length >= limits.2 
        || cost > *dist[position].get(&length).unwrap() {
            continue;
        }

        for flight in &cities[position].edges {
            let next = State {
                cost: cost + flight.cost,
                length: length + 1, 
                position: flight.to
            };

            if next.cost < *dist[next.position].get(&next.length).unwrap() {
                dist[next.position].insert(next.length, next.cost);
                heap.push(next);
            }
        }
    }

    let mut min = u32::MAX;

    for i in dist[limits.1].values() {
        if i < &min { min = *i; }
    }

    match min {
        u32::MAX => None,
        _ => Some(min)
    }
}