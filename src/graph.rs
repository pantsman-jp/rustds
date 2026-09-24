use crate::queue::Queue;

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
    // add directed edge from `u` to `v`
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adjacency_list[u].push(v);
    }

    pub fn neighbors(&self, u: usize) -> &[usize] {
        &self.adjacency_list[u]
    }

    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut q = Queue::new();
        let mut dist = vec![usize::MAX; self.adjacency_list.len()];
        dist[start] = 0;
        q.push(start);
        while !q.is_empty() {
            let u = q.pop().unwrap();
            for &v in self.neighbors(u) {
                if dist[v] != usize::MAX {
                    continue;
                } else {
                    dist[v] = dist[u] + 1;
                    q.push(v);
                }
            }
        }
        dist
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

    #[test]
    fn test_bfs() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(3, 2);
        let inf = usize::MAX;
        assert_eq!(g.bfs(0), vec![0, 1, 1, 2]);
        assert_eq!(g.bfs(1), vec![inf, 0, 2, 1]);
        assert_eq!(g.bfs(2), vec![inf, inf, 0, inf]);
        assert_eq!(g.bfs(3), vec![inf, inf, 1, 0]);
    }
}
