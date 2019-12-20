use std::collections::{VecDeque, HashMap};
use std::hash::Hash;
use std::iter;

enum NodeIterType {
  Bfs,
  Dfs
}

pub struct PredecessorIter<'a, G: Graph<V>, V: Hash + Eq + Copy> {
  node_iter: NodeIter<'a, G, V>
}

impl<'a, G: Graph<V>, V: Hash + Eq + Copy> PredecessorIter<'a, G, V> {
  pub fn construct_path(&mut self, target: V) -> Option<Vec<V>> {
    self.node_iter.construct_path(target)
  }
}

impl<'a, G: Graph<V>, V: Hash + Eq + Copy> Iterator for PredecessorIter<'a, G, V> {
  type Item = (Option<V>, V);

  fn next(&mut self) -> Option<Self::Item> {
    self.node_iter.next()
    .and_then(|v| {
      self.node_iter.predecessor_map.get(&v).map(|&p| (p, v))
    })
  }
}

pub struct NodeIter<'a, G: Graph<V>, V: Hash + Eq + Copy> {
  graph: &'a G,
  start: V,
  queue: VecDeque<V>,
  predecessor_map: HashMap<V, Option<V>>,
  iter_type: NodeIterType
}

impl<'a, G: Graph<V>, V: Hash + Eq + Copy> NodeIter<'a, G, V> {
  pub fn predecessor(self) -> PredecessorIter<'a, G, V> where Self: Sized {
    PredecessorIter {
      node_iter: self
    }
  }

  pub fn construct_path(&mut self, target: V) -> Option<Vec<V>> {
    if !self.predecessor_map.contains_key(&target) {
      self.find(|&v| v == target);
    }

    let mut path = vec![target];

    while let Some(&Some(previous)) = self.predecessor_map.get(path.last().unwrap()) {
      path.push(previous);
    }

    path.reverse();

    if path[0] == self.start {
      Some(path)
    } else {
      None
    }
  }
}

impl<'a, G: Graph<V>, V: Hash + Eq + Copy> Iterator for NodeIter<'a, G, V> {
  type Item = V;

  fn next(&mut self) -> Option<Self::Item> {
    let vertex = match self.iter_type {
      NodeIterType::Bfs => self.queue.pop_front(),
      NodeIterType::Dfs => self.queue.pop_back()
    };

    vertex.map(|vertex| {
      for neighbor in self.graph.get_neighbors(vertex) {
        if self.predecessor_map.contains_key(&neighbor) {
          continue;
        }

        self.queue.push_back(neighbor);
        self.predecessor_map.insert(neighbor, Some(vertex));
      }

      vertex
    })
  }
}

pub trait Graph<V: Hash + Eq + Copy> {
  fn get_neighbors(&self, vertex: V) -> Vec<V>;

  fn bfs<'a>(&'a self, start: V) -> NodeIter<'a, Self, V> where Self: Sized {
    NodeIter {
      graph: self,
      start,
      queue: iter::once(start).collect(),
      predecessor_map: iter::once((start, None)).collect(),
      iter_type: NodeIterType::Bfs
    }
  }

  fn dfs<'a>(&'a self, start: V) -> NodeIter<'a, Self, V> where Self: Sized {
    NodeIter {
      graph: self,
      start,
      queue: iter::once(start).collect(),
      predecessor_map: iter::once((start, None)).collect(),
      iter_type: NodeIterType::Dfs
    }
  }
}
