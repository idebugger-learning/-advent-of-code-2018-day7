use std::collections::{HashSet, VecDeque};

use petgraph::{graphmap::DiGraphMap, Direction::Incoming};
use regex::Regex;

#[derive(Debug)]
struct Worker {
    chr: char,
    seconds_left: u32,
}

fn main() {
    let input = include_str!("./data/input.txt");
    let graph = parse_input(input);

    let order = run_search(&graph);
    println!("Order: {:?}", order.iter().collect::<String>());

    let time_search = run_timed_search(&graph, 5, 61);
    println!("Timed search: {:?}", time_search);
}

fn parse_input(input: &str) -> DiGraphMap<char, ()> {
    let mut graph = DiGraphMap::new();

    let re = Regex::new(r"^Step (.) must be finished before step (.) can begin.$").unwrap();
    for line in input.split('\n') {
        let captures = re.captures(line).expect("Failed to find captures");

        let from = captures
            .get(1)
            .expect("Can't get first match")
            .as_str()
            .chars()
            .next()
            .expect("Can't get letter for 'from' match");
        let to = captures
            .get(2)
            .expect("Cant' get second match")
            .as_str()
            .chars()
            .next()
            .expect("Can't get letter for 'to' match");

        if !graph.contains_node(from) {
            graph.add_node(from);
        }
        if !graph.contains_node(to) {
            graph.add_node(to);
        }

        graph.add_edge(from, to, ());
    }

    graph
}

fn run_search(graph: &DiGraphMap<char, ()>) -> Vec<char> {
    let mut order = vec![];

    let mut graph = graph.clone();
    while graph.node_count() > 0 {
        let mut no_incoming_nodes = graph
            .nodes()
            .filter(|edge| graph.neighbors_directed(*edge, Incoming).next().is_none())
            .collect::<Vec<_>>();
        no_incoming_nodes.sort();
        let node_to_process = no_incoming_nodes[0];
        order.push(node_to_process);
        graph.remove_node(node_to_process);
    }

    order
}

fn run_timed_search(graph: &DiGraphMap<char, ()>, max_workers: usize, min_time: u32) -> u32 {
    let mut time: u32 = 0;
    let mut workers: Vec<Worker> = vec![];

    let mut graph = graph.clone();
    while graph.node_count() > 0 || workers.len() > 0 {
        time += 1;

        // The whole function is so bad...

        for worker in &mut workers {
            if worker.seconds_left == 1 {
                graph.remove_node(worker.chr);
            }
        }

        workers = workers
            .into_iter()
            .filter(|worker| worker.seconds_left != 1)
            .collect::<Vec<_>>();

        for worker in &mut workers {
            worker.seconds_left -= 1;
        }

        let mut no_incoming_nodes = graph
            .nodes()
            .filter(|edge| graph.neighbors_directed(*edge, Incoming).next().is_none())
            .filter(|edge| workers.iter().find(|worker| worker.chr == *edge).is_none())
            .collect::<Vec<_>>();
        no_incoming_nodes.sort();
        let mut no_incoming_nodes = no_incoming_nodes.iter().collect::<VecDeque<_>>();

        while workers.len() < max_workers && no_incoming_nodes.len() > 0 {
            let node_to_process = *no_incoming_nodes
                .pop_front()
                .expect("Failed to get next node");
            let time_to_process = node_to_process as u32 - 'A' as u32 + min_time;
            workers.push(Worker {
                chr: node_to_process,
                seconds_left: time_to_process,
            });
        }
    }

    time
}
