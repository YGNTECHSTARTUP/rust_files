use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut graph: HashMap<String, HashMap<String, f64>> = HashMap::new();

    graph.insert(
        "start".to_string(),
        HashMap::from([("a".to_string(), 6.0), ("b".to_string(), 2.0)]),
    );
    graph.insert("a".to_string(), HashMap::from([("fin".to_string(), 1.0)]));
    graph.insert(
        "b".to_string(),
        HashMap::from([("a".to_string(), 3.0), ("fin".to_string(), 5.0)]),
    );
    graph.insert("fin".to_string(), HashMap::new());

    let (costs, parents) = dijkstra(&graph, "start".to_string());

    println!("Costs: {:?}", costs);
    println!("Parents: {:?}", parents);
}

fn dijkstra(
    graph: &HashMap<String, HashMap<String, f64>>,
    start: String,
) -> (HashMap<String, f64>, HashMap<String, Option<String>>) {
    let mut costs: HashMap<String, f64> = HashMap::new();
    let mut parent: HashMap<String, Option<String>> = HashMap::new();
    let mut processed: HashSet<String> = HashSet::new();
    for node in graph.keys() {
        if *node == start {
            costs.insert(node.clone(), 0.0);
        } else {
            costs.insert(node.clone(), f64::INFINITY);
        }
        parent.insert(node.clone(), None);
    }
    let mut node = smallest_node(&costs, &processed).cloned();
    while let Some(current_node) = node {
        if let Some(neighbors) = graph.get(&current_node) {
            for (node, &edge_cost) in neighbors {
                let cost = *costs.get(&current_node).unwrap_or(&f64::INFINITY);
                let new_cost = cost + edge_cost;
                if new_cost < *costs.get(node).unwrap_or(&f64::INFINITY) {
                    costs.insert(node.clone(), new_cost);
                    parent.insert(node.clone(), Some(current_node.clone()));
                }
            }
        }
        processed.insert(current_node.clone());
        node = smallest_node(&costs, &processed).cloned();
    }
    (costs, parent)
}

fn smallest_node<'a>(costs: &'a HashMap<String, f64>, processed: &'a HashSet<String>) -> Option<&'a String> {
    let mut smallest_cost = f64::INFINITY;
    let mut smallest_node = None;
    for (node, &cost) in costs.iter() {
        if cost < smallest_cost && !processed.contains(node) {
            smallest_cost = cost;
            smallest_node = Some(node);
        }
    }
    smallest_node
}

// fn sum(a: &[u32]) -> u32 {
//     if a.len() < 2 {
//         return a[0];
//     } else {
//         println!("{:?}", &a);
//         return a[0] + sum(&a[1..]);
//     }
// }

// fn binary_search(a: &[u32], term: u32, left: u32, right: u32) -> bool {
//     if left > right {
//         return false;
//     } else {
//         let mid = left + (right - left) / 2;
//         if let Some(&m) = a.get(mid as usize) {
//             match m.cmp(&term) {
//                 std::cmp::Ordering::Less => binary_search(a, term, mid + 1, right),
//                 std::cmp::Ordering::Greater => binary_search(a, term, left, mid - 1),
//                 std::cmp::Ordering::Equal => {
//                     return true;
//                 }
//             }
//         } else {
//             return false;
//         }
//     }
// }

// type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

// fn bfs(graph: &Graph, start: &str, end: &str) -> bool {
//     let mut visited: HashSet<&str> = HashSet::new();
//     let mut queue: VecDeque<&str> = VecDeque::new();
//     queue.push_back(start);
//     visited.insert(start);
//     while let Some(current) = queue.pop_front() {
//         if current == end {
//             return true;
//         }
//         if let Some(neighbours) = graph.get(current) {
//             for &neighbour in neighbours {
//                 if !visited.contains(neighbour) {
//                     queue.push_back(neighbour);
//                     visited.insert(neighbour);
//                 }
//             }
//         }
//     }
//     return false;
// }

// fn shortest_bfs<'a>(graph: &'a Graph, start: &'a str, end: &'a str) -> Vec<&'a str> {
//     let mut visited = HashSet::new();
//     let mut queue = VecDeque::new();
//     let mut path: Vec<&str> = vec![];
//     let mut parent: HashMap<&str, Option<&str>> = HashMap::new();
//     queue.push_back(start);
//     visited.insert(start);
//     parent.insert(start, None);
//     while let Some(current) = queue.pop_front() {
//         let mut current = current;
//         if current == end {
//             path.push(current);
//             while let Some(&Some(pare)) = parent.get(current) {
//                 path.push(pare);
//                 current = pare;
//             }
//             path.reverse();
//             return path;
//         } else {
//             if let Some(neighbours) = graph.get(current) {
//                 for &neighbour in neighbours {
//                     if !visited.contains(neighbour) {
//                         queue.push_back(neighbour);
//                         visited.insert(neighbour);
//                         parent.insert(neighbour, Some(current));
//                     }
//                 }
//             }
//         }
//     }
//     Vec::new()
// }

// // fn sum(mut a: Vec<u32>) -> u32 {

// //     if a.len() == 1 {
// //         return a.pop().unwrap();
// //     } else {
// //         a.pop().unwrap() + sum(a)
// //     }
// // }

// // fn count(mut a: Vec<u32>) -> u32 {
// //     if a.len() == 1 {
// //         return 1;
// //     } else {
// //         a.pop().is_some() as u32 + count(a)
// //     }
// // }

// // fn max_num(a: &[u32]) -> Option<u32> {
// //     if a.is_empty() {
// //         None
// //     } else if a.len() == 1 {
// //         Some(a[0])
// //     } else {
// //         let rest = max_num(&a[1..]);
// //         if a[0] < rest.unwrap() {
// //             rest
// //         } else {
// //             Some(a[0])
// //         }
// //     }
// // }

// // fn bs(a: &[u32], b: u32, first: usize, last: usize) -> bool {
// //     if first > last {
// //         return false;
// //     }
// //     let mid = first + (last - first) / 2;
// //     if a[mid] == b {
// //         return true;
// //     } else if a[mid] > b {
// //         if mid == 0 {
// //             return false;
// //         }
// //         bs(a, b, first, mid - 1)
// //     } else {
// //         bs(a, b, mid + 1, last)
// //     }
// // }

// fn quick_sort(a: &[u32]) -> Vec<u32> {
//     let len = a.len();
//     if len < 2 {
//         return a.to_vec();
//     } else {
//         let pivot = a[0];
//         let less: Vec<u32> = a[1..].iter().cloned().filter(|&x| x < pivot).collect();

//         let greater: Vec<u32> = a[1..].iter().cloned().filter(|&x| x > pivot).collect();
//         let mut result = quick_sort(&less);
//         result.push(pivot);
//         result.extend(quick_sort(&greater));
//         return result;
//     }
// }
// fn merge_sort(a: &[u32], i: u32, j: u32) -> Vec<u32> {
//     let len = a.len();
//     if len < 2 {
//         return a.to_vec();
//     } else {
//         return a.to_vec();
//     }
// }

// fn hashmap() {
//     let mut a = HashMap::new();
// }
