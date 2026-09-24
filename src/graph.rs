pub struct Graph {
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            adjacency_list: vec![Vec::new(); n],
        }
    }

    // 0-index
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adjacency_list[u].push(v);
    }

    pub fn neighbors(&self, u: usize) -> &[usize] {
        &self.adjacency_list[u]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        assert_eq!(g.neighbors(0), vec![1]);
        g.add_edge(1, 1);
        assert_eq!(g.neighbors(1), vec![1]);
        g.add_edge(0, 2);
        assert_eq!(g.neighbors(0), vec![1, 2]);
    }
}
