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
}
