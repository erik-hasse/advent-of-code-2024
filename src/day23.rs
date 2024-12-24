use anyhow::anyhow;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::iter::once;
use std::path::Path;

fn read_to_string(input: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(input).map_err(Into::into)
}

type Id = (char, char);

type Edges = HashMap<Id, HashSet<Id>>;

#[derive(Eq, PartialEq, Debug)]
struct Clique(HashSet<Id>);

impl Hash for Clique {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // To ensure a consistent order, hash the elements in sorted order
        let mut elements: Vec<_> = self.0.iter().collect();
        elements.sort();
        for element in elements {
            element.hash(state);
        }
    }
}

impl Clique {
    fn contains_possible(&self) -> bool {
        self.0.iter().any(|x| x.0 == 't')
    }
}

fn find_groups(edges: Edges) -> HashSet<Clique> {
    let mut groups = HashSet::new();
    for (v, neighbors) in edges.iter() {
        for (n1, n2) in neighbors.iter().tuple_combinations() {
            let triple = edges.get(n1).map(|ns| ns.contains(n2)).unwrap_or(false);
            if triple {
                groups.insert(Clique(HashSet::from_iter([v, n1, n2].into_iter().copied())));
            }
        }
    }
    groups
}

fn build_id(s: &str) -> Id {
    (s.chars().next().unwrap(), s.chars().nth(1).unwrap())
}

fn build_edges(s: String) -> Edges {
    let mut res = HashMap::new();
    for l in s.lines() {
        let (s1, s2) = l.split_once("-").unwrap();
        let id1 = build_id(s1);
        let id2 = build_id(s2);
        res.entry(id1)
            .and_modify(|e: &mut HashSet<Id>| {
                e.insert(id2);
            })
            .or_insert(HashSet::from_iter([id2]));
        res.entry(id2)
            .and_modify(|e: &mut HashSet<Id>| {
                e.insert(id1);
            })
            .or_insert(HashSet::from_iter([id1]));
    }
    res
}

pub fn part_a(input: &Path) -> anyhow::Result<usize> {
    let s = read_to_string(input)?;
    let edges = build_edges(s);
    let groups = find_groups(edges);
    let groups: Vec<&Clique> = groups.iter().filter(|c| c.contains_possible()).collect();

    println!("{:?}", groups);
    Ok(groups.len())
}

#[derive(Clone)]
struct Graph {
    nodes: Vec<Id>,
    edges: HashSet<(Id, Id)>,
}

impl Graph {
    fn new(s: String) -> Self {
        let mut nodes = HashSet::new();
        let mut edges = HashSet::new();
        for l in s.lines() {
            let (id1, id2) = l
                .split_once("-")
                .map(|(s1, s2)| (build_id(s1), build_id(s2)))
                .unwrap();
            nodes.insert(id1);
            nodes.insert(id2);
            edges.insert((id1, id2));
            edges.insert((id2, id1));
        }
        let nodes = nodes.into_iter().sorted().collect();

        Self { nodes, edges }
    }

    fn neighbors(&self, id: Id) -> HashSet<Id> {
        HashSet::from_iter(once(id).chain(self.edges.iter().filter_map(|&e| {
            if e.0 == id {
                Some(e.1)
            } else if e.1 == id {
                Some(e.0)
            } else {
                None
            }
        })))
    }
}

pub fn part_b(input: &Path) -> anyhow::Result<String> {
    let s = read_to_string(input)?;
    let g = Graph::new(s);
    let mut hq = BinaryHeap::new();
    let mut hm: HashMap<usize, HashSet<Id>> = HashMap::new();
    for id in g.nodes.iter() {
        let ns = g.neighbors(*id);
        let key = hm.keys().max().unwrap_or(&0) + 1;
        let len = ns.len();
        hm.insert(key, ns);
        hq.push((len, key))
    }

    while let Some((s, key)) = hq.pop() {
        let f = hm.get(&key).unwrap().clone();
        if f.iter()
            .tuple_combinations()
            .all(|(&id1, &id2)| g.edges.contains(&(id1, id2)))
        {
            return Ok(f
                .iter()
                .sorted()
                .map(|(a, b)| [a, b].iter().join(""))
                .join(","));
        }
        for n in f.iter() {
            let key = hm.keys().max().unwrap_or(&0) + 1;

            hm.insert(
                key,
                HashSet::from_iter(f.iter().filter(|&k| k != n).copied()),
            );
            hq.push((s - 1, key));
        }
    }

    Err(anyhow!("Not found!"))
}
