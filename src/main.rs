use petgraph::{graphmap::DiGraphMap, Direction::Incoming};
use regex::Regex;

fn main() {
    let input = include_str!("./data/input_example.txt");
    let graph = parse_input(input);

    let order = run_search(&graph);
    println!("Order: {:?}", order.iter().collect::<String>());

    let time_search = run_timed_search(&graph);
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

fn run_timed_search(graph: &DiGraphMap<char, ()>) -> u32 {
    let mut time: u32 = 0;

    let mut graph = graph.clone();
    while graph.node_count() > 0 {
        let mut no_incoming_nodes = graph
            .nodes()
            .filter(|edge| graph.neighbors_directed(*edge, Incoming).next().is_none())
            .collect::<Vec<_>>();
        no_incoming_nodes.sort();

        let min_node = no_incoming_nodes
            .iter()
            .take(2)
            .min()
            .expect("Can't find minimum node");

        let time_add = *min_node as u32 - 'A' as u32 + 1;
        println!("{} - {}", min_node, time_add);
        time += time_add;

        graph.remove_node(*min_node);
    }

    time
}
