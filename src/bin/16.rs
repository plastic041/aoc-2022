use std::collections::{HashMap, HashSet, VecDeque};

use nom::IResult;

type Routes<'a> = HashMap<&'a str, HashMap<&'a str, i32>>;

#[derive(Debug, Clone)]
struct Node<'a> {
    id: &'a str,
    flow_rate: i32,
    connections: Vec<&'a str>,
}

impl<'a> Node<'a> {
    fn new(id: &'a str, flow_rate: i32, connections: Vec<&'a str>) -> Self {
        Self {
            id,
            flow_rate,
            connections,
        }
    }

    /// Parse a line of input into a Node.
    /// "Valve [id] has flow rate=[flow_rate]; tunnels lead to valves [connections]"
    fn parse_line(input: &'a str) -> IResult<&str, Self> {
        let (input, _) = nom::bytes::complete::tag("Valve ")(input)?;
        let (input, id) = nom::bytes::complete::take_while(|c| c != ' ')(input)?;
        let (input, _) = nom::bytes::complete::tag(" has flow rate=")(input)?;
        let (input, flow_rate) = nom::character::complete::i32(input)?;
        let (input, _) = nom::bytes::complete::tag("; tunnels lead to valves ")(input)?;
        let (input, connections) = nom::multi::separated_list0(
            nom::bytes::complete::tag(", "),
            nom::character::complete::alpha1,
        )(input)?;

        Ok((input, Self::new(id, flow_rate, connections)))
    }

    /// Breadth-first search to find the shortest path to another node.
    fn bfs(&self, nodes: &[Node], end: &Node) -> Option<i32> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((self.id, 0));

        while let Some((id, depth)) = queue.pop_front() {
            if id == end.id {
                return Some(depth);
            }

            if visited.contains(&id) {
                continue;
            }

            visited.insert(id);

            for node in nodes {
                if node.id == id {
                    continue;
                }

                if node.connections.contains(&id) {
                    queue.push_back((node.id, depth + 1));
                }
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let nodes = input
        .lines()
        .map(|line| Node::parse_line(line).unwrap().1)
        .collect::<Vec<_>>();

    let flow_rates: HashMap<&str, i32> =
        nodes.iter().map(|node| (node.id, node.flow_rate)).collect();

    let mut routes: Routes = HashMap::new();
    for node in &nodes {
        let mut route: HashMap<&str, i32> = HashMap::new();

        for other in &nodes {
            if node.id == other.id {
                continue;
            }

            if let Some(path) = node.bfs(&nodes, other) {
                let other_flow_rate = flow_rates[&other.id];
                if other_flow_rate != 0 {
                    route.insert(other.id, path);
                }
            }
        }

        if (!route.is_empty() && node.flow_rate != 0) || node.id == "AA" {
            routes.insert(node.id, route.clone());
        }
    }

    let key_valves = nodes
        .iter()
        .filter(|node| node.flow_rate != 0)
        .map(|node| node.id)
        .collect::<Vec<_>>();

    let mut seen: HashSet<&str> = HashSet::new();

    let best_flow = find_best_total_flow("AA", 30, &mut seen, key_valves, &routes, &flow_rates);

    Some(best_flow)
}

// https://topaz.github.io/paste/#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
fn find_best_total_flow(
    current_id: &str,
    time: i32,
    seen: &mut HashSet<&str>,
    targets: Vec<&str>,
    routes: &Routes,
    flow_rates: &HashMap<&str, i32>,
) -> i32 {
    let mut new_seen = seen.clone();
    new_seen.insert(current_id);

    // remove 'seen's from target
    let targets = targets
        .into_iter()
        .filter(|target| !new_seen.contains(*target))
        .collect::<Vec<_>>();

    println!("{}: {} {:?}", time, current_id, targets);

    let mut best_flow = 0;
    for target in &targets {
        let time_left = time - routes[current_id][*target] - 1;
        if time_left > 0 {
            let mut flow = flow_rates[*target] * time_left;
            flow += find_best_total_flow(
                target,
                time_left,
                &mut new_seen,
                targets.clone(),
                routes,
                flow_rates,
            );
            best_flow = best_flow.max(flow);
        }
    }

    best_flow
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
