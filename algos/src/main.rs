use std::{
    collections::{HashMap, HashSet, VecDeque},
    thread::current,
};

fn main() {}

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn bfs(graph: &Graph, start: &str, end: &str) {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back(start);
    visited.insert(start);
    while let Some(current) = queue.pop_front() {
        printl
        if current == end {
            println!("Found");
        }
        while let Some(neighbours) = graph.get(current) {
            for &neighbour in neighbours {
                queue.push_back(neighbour);
                visited.insert(neighbour);
            }
        }
    }
}

// fn sum(mut a: Vec<u32>) -> u32 {

//     if a.len() == 1 {
//         return a.pop().unwrap();
//     } else {
//         a.pop().unwrap() + sum(a)
//     }
// }

// fn count(mut a: Vec<u32>) -> u32 {
//     if a.len() == 1 {
//         return 1;
//     } else {
//         a.pop().is_some() as u32 + count(a)
//     }
// }

// fn max_num(a: &[u32]) -> Option<u32> {
//     if a.is_empty() {
//         None
//     } else if a.len() == 1 {
//         Some(a[0])
//     } else {
//         let rest = max_num(&a[1..]);
//         if a[0] < rest.unwrap() {
//             rest
//         } else {
//             Some(a[0])
//         }
//     }
// }

// fn bs(a: &[u32], b: u32, first: usize, last: usize) -> bool {
//     if first > last {
//         return false;
//     }
//     let mid = first + (last - first) / 2;
//     if a[mid] == b {
//         return true;
//     } else if a[mid] > b {
//         if mid == 0 {
//             return false;
//         }
//         bs(a, b, first, mid - 1)
//     } else {
//         bs(a, b, mid + 1, last)
//     }
// }

fn quick_sort(a: &[u32]) -> Vec<u32> {
    let len = a.len();
    if len < 2 {
        return a.to_vec();
    } else {
        let pivot = a[0];
        let less: Vec<u32> = a[1..].iter().cloned().filter(|&x| x < pivot).collect();

        let greater: Vec<u32> = a[1..].iter().cloned().filter(|&x| x > pivot).collect();
        let mut result = quick_sort(&less);
        result.push(pivot);
        result.extend(quick_sort(&greater));
        println!("{:?}", less);
        return result;
    }
}

fn hashmap() {
    let mut a = HashMap::new();
    a.insert(1, "ballaya");
    println!("{:?}", a);
}

struct GroceryStore {
    basket: HashMap<String, f64>,
}

impl GroceryStore {
    pub fn new() -> GroceryStore {
        GroceryStore {
            basket: HashMap::new(),
        }
    }
    pub fn get_price(&self, item: &str) -> f64 {
        *self.basket.get(item).unwrap()
    }
    pub fn insert(&mut self, item: &str, price: f64) {
        self.basket.insert(item.to_string(), price);
    }
}

struct Network {}
