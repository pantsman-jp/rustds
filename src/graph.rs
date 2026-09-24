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
}

#[cfg(test)]
mod tests {
    use super::*;
}
