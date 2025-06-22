#[derive(Copy, Clone, Debug)]
pub enum BondOrder {
    Single,
    Double,
    Triple,
}

#[derive(Debug)]
pub struct Graph {
    pub elements: Vec<String>,
    pub edge_offsets: Vec<usize>,
    pub edge_targets: Vec<usize>,
    pub bond_orders: Vec<BondOrder>,
}

impl Graph {
    pub fn new(
        num_atoms: usize,
        edges: Vec<(usize, usize, BondOrder)>,
        elements: Vec<String>,
    ) -> Self {
        // Build CSR adjacency from edge list
        let mut edge_offsets = vec![0; num_atoms + 1];
        for (src, _, _) in &edges {
            edge_offsets[src + 1] += 1;
        }
        for i in 1..=num_atoms {
            edge_offsets[i] += edge_offsets[i - 1];
        }
        let mut edge_targets = vec![0; edges.len()];
        let mut bond_orders = vec![BondOrder::Single; edges.len()];
        let mut next = edge_offsets.clone();
        for (src, dst, order) in edges {
            let idx = next[src];
            edge_targets[idx] = dst;
            bond_orders[idx] = order;
            next[src] += 1;
        }
        Graph {
            elements,
            edge_offsets,
            edge_targets,
            bond_orders,
        }
    }

    pub fn neighbors(&self, node: usize) -> impl Iterator<Item = (usize, BondOrder)> + '_ {
        let start = self.edge_offsets[node];
        let end = self.edge_offsets[node + 1];
        self.edge_targets[start..end]
            .iter()
            .cloned()
            .zip(self.bond_orders[start..end].iter().cloned())
    }
}
