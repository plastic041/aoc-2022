use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use nom::IResult;

type Routes = HashMap<String, HashMap<String, usize>>;

#[derive(Debug, Clone)]
struct Node {
    id: String,
    flow_rate: u32,
    connections: Vec<String>,
}

impl Node {
    fn new(id: String, flow_rate: u32, connections: Vec<String>) -> Self {
        Self {
            id,
            flow_rate,
            connections,
        }
    }

    /// Parse a line of input into a Node.
    /// "Valve [id] has flow rate=[flow_rate]; tunnels lead to valves [connections]"
    fn parse_line(input: &str) -> IResult<&str, Self> {
        let (input, _) = nom::bytes::complete::tag("Valve ")(input)?;
        let (input, id) = nom::bytes::complete::take_while(|c| c != ' ')(input)?;
        let (input, _) = nom::bytes::complete::tag(" has flow rate=")(input)?;
        let (input, flow_rate) = nom::character::complete::u32(input)?;
        let (input, _) = nom::bytes::complete::tag("; tunnels lead to valves ")(input)?;
        let (input, connections) = nom::multi::separated_list0(
            nom::bytes::complete::tag(", "),
            nom::character::complete::alpha1,
        )(input)?;

        Ok((
            input,
            Self::new(
                id.to_string(),
                flow_rate,
                connections.iter().map(|s| s.to_string()).collect(),
            ),
        ))
    }

    /// Breadth-first search to find the shortest path to another node.
    fn bfs(&self, nodes: &[Node], end: &Node) -> Option<usize> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((self.id.clone(), 0));

        while let Some((id, depth)) = queue.pop_front() {
            if id == end.id {
                return Some(depth);
            }

            if visited.contains(&id) {
                continue;
            }

            visited.insert(id.clone());

            for node in nodes {
                if node.id == id {
                    continue;
                }

                if node.connections.contains(&id) {
                    queue.push_back((node.id.clone(), depth + 1));
                }
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let nodes = input
        .lines()
        .map(|line| Node::parse_line(line).unwrap().1)
        .collect::<Vec<_>>();

    let flow_rates: HashMap<String, u32> = nodes
        .iter()
        .map(|node| (node.id.clone(), node.flow_rate))
        .collect();

    let mut a_routes = HashMap::new();
    let mut routes: Routes = HashMap::new();
    for node in &nodes {
        let mut route = HashMap::new();

        for other in &nodes {
            if node.id == other.id {
                continue;
            }

            if let Some(path) = node.bfs(&nodes, other) {
                let other_flow_rate = flow_rates[&other.id];
                if other_flow_rate != 0 {
                    route.insert(other.id.clone(), path);
                }
            }
        }

        if (!route.is_empty() && node.flow_rate != 0) || node.id == "AA" {
            routes.insert(node.id.clone(), route.clone());
        }

        if node.id == "AA" {
            a_routes = route;
        }
    }

    let key_valves = nodes
        .iter()
        .filter(|node| node.flow_rate != 0)
        .map(|node| node.id.clone())
        .collect::<Vec<_>>();

    let mut debug_count: i64 = 0;
    let perms = a_routes
        .iter()
        .map(|(id, _)| id.clone())
        .permutations(a_routes.len())
        .map(|mut perm| {
            perm.insert(0, "AA".to_string());

            debug_count += 1;
            if debug_count % 100000 == 0 {
                println!("{} permutations", debug_count);
            }
            perm
        });

    let mut max_flows = 0;
    for perm in perms {
        let mut time = a_routes[&perm[1]] + 1;
        let mut prev_time = time;
        let mut flows_curr = flow_rates[&perm[1]];

        let mut flows_acc = 0;

        let mut prev_id = &perm[1].clone();

        for id in perm[2..].iter() {
            if prev_id.is_empty() {
                prev_id = id;
                continue;
            }
            let time_takes = routes[prev_id][id] + 1;
            time += time_takes;
            if time > 30 {
                break;
            }

            flows_acc += flows_curr * (time - prev_time) as u32;
            // "DD"
            let flow_rate = flow_rates[id];
            flows_curr += flow_rate;

            if perm.last().unwrap() == id {
                flows_acc += flows_curr * (30 - time) as u32;
                dbg!(time, flows_curr, flows_acc);
                break;
            }

            prev_id = id;

            prev_time = time;
        }
        max_flows = max_flows.max(flows_acc);
    }

    Some(max_flows)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_node_parse() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let (input, node) = Node::parse_line(input).unwrap();

        assert_eq!(input, "");
        assert_eq!(node.id, "AA");
        assert_eq!(node.flow_rate, 0);
        assert_eq!(node.connections, vec!["DD", "II", "BB"]);
    }
}
