pub struct Graph {
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            adjacency_list: vec![Vec::new(); n],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
