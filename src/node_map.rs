use regex::Regex;

/// Map from node indices to netlist
/// node names
#[derive(Debug)]
pub struct NodeMap {
    /// Voltage nodes (including ground at position 0)
    index_to_name: Vec<String>,
    /// Current edge labels
    edge_to_name: Vec<String>,
}

impl NodeMap {
    /// Make an empty node map
    pub fn new() -> Self {
        Self {
            index_to_name: vec![String::from("")],
            edge_to_name: vec![],
        }
    }

    fn add_ground_node(&mut self, ground_name: &str) {
        if self.index_to_name[0] == "" {
            // If no ground node has been encountered yet,
            // store it here
            self.index_to_name[0] = String::from(ground_name);
        } else {
            // Else, check the ground name agrees with the
            // previously used name
            if self.index_to_name[0] != ground_name {
                panic!(
                    "Ground node name mismatch: expected {}, found {}",
                    self.index_to_name[0], ground_name
                );
            }
        }
    }

    /// Assign a terminal string to a new index, or return the index
    /// if it was already assigned.
    pub fn node_index(&mut self, node_name: &str) -> usize {
        let re = Regex::new(r"(gnd|GND|0)").unwrap();
        if re.is_match(node_name) {
            self.add_ground_node(node_name);
            0
        } else if let Some(result) = self.index_to_name.iter().position(|s| s == node_name) {
            result
        } else {
            self.index_to_name.push(String::from(node_name));
            self.index_to_name.len() - 1
        }
    }

    /// Assign a terminal string to a new index, or return the index
    /// if it was already assigned.
    pub fn edge_index(&mut self, edge_name: &str) -> usize {
        if let Some(result) = self.edge_to_name.iter().position(|s| s == edge_name) {
            result
        } else {
            self.edge_to_name.push(String::from(edge_name));
            self.edge_to_name.len() - 1
        }
    }
    
    pub fn node_name(&self, index: usize) -> &String {
        &self.index_to_name[index]
    }

    pub fn edge_name(&self, index: usize) -> &String {
        &self.edge_to_name[index]
    }
}
