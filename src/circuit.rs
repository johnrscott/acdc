pub mod element;
pub mod instance;
mod index_map;

use crate::circuit::element::Element;
use crate::sparse::{SparseMatrix, ColumnVector,
		    concat_horizontal, concat_vertical,
		    transpose};
use crate::circuit::index_map::IndexMap;
use crate::circuit::instance::Instance;

/// Matrix for modified nodal analysis
///
/// Stores the modified nodal analysis matrix
/// for a resistive network with no controlled,
/// sources, where group 2 contains no current
/// sources.
///
///  | A1 Y11 A1^T     A2  |
///  |                     |
///  |   - A2         Z22  |
/// 
///
struct MnaMatrix<P: ValueType<P>> {
    a1_y11_a1t: SparseMatrix<P>,
    a2: SparseMatrix<P>,
    z22: SparseMatrix<P>,
}

/// Modified nodal analysis right-hand side
///
/// The right-hand side for modified nodal analysis is
///
/// | -A1 s1 |
/// |        |
/// |   s2   |
///
struct MnaRhs {
    minus_a1_s1: Vec<P>,
    s2: Vec<P>,
}

impl MnaMatrix {
    pub fn new() -> Self {
	Self {
	    a1_y11_a1t: SparseMatrix::new(),
	    a2: SparseMatrix::new(),
	    z22: SparseMatrix::new(),
	}
    }
    pub fn get_matrix(self) -> SparseMatrix {
	let top = concat_horizontal(self.A1Y11A1t, &self.A2);
	let bottom = concat_horizontal(-transpose(self.A2), &self.Z22);
	concat_vertical(top, &bottom)
    }
    pub fn insert_group1(&self, row: usize, col: usize, value: f64) {
	self.a1_y11_A1t.set_value(row, col, value);
    }
    pub fn insert_group2(&self, row: usize, col: usize, value: f64) {
	self.a1_y11_a1t.set_value(row, col, value);
    }
}

impl MnaRhs {
    fn new() -> Self {
	Self {
	    top: ColumnVector::new(),
	    bottom: ColumnVector::new(),
	}
    }
    pub fn get_vector(mut self) -> Vec<P> {
	self.top.append(self.bottom);
	self.top
    }
}

pub struct Circuit {
    instances: Vec<Instance>,
    node_index_map: IndexMap,
    element_index_map: IndexMap,
    mna_matrix: MnaMatrix,
    mna_rhs: MnaRhs,
}

impl Circuit {
   pub fn new() -> Self {
	Self {
	    instances: Vec::new(),
	    node_index_map: IndexMap::new(1),
	    element_index_map: IndexMap::new(0),
	    mna_matrix: MnaMatrix::new(),
	    mna_rhs: MnaRhs::new(),
	}
    }
    fn update_node_index_map(&mut self, terminal_names: Vec<String>)
			     -> Vec<usize> {
	let mut node_indices = Vec::new();
	for node in terminal_names {
	    let mut index = 0;
	    if node.contains("gnd") {
		self.node_index_map.insert_at(0, node);
	    } else if !self.node_index_map.contains_key(&node) {
		index = self.node_index_map.insert(node);
	    }
	    node_indices.push(index)
	}
	node_indices
    }
    fn update_element_index_map(&mut self, element_name: String) -> usize {
	self.element_index_map.insert(element_name)
    }
    fn add_element_stamp(&mut self,
			 instance: &Instance,
			 node_indices: Vec<usize>,
			 element_index: usize) {
	instance.element.add_stamp(&mut self.mna_matrix,
				   &mut self.mna_rhs,
				   node_indices,
				   element_index,
				   instance.group2);
    }
    pub fn add_new_instance(&mut self, instance: Instance) {
	let terminal_names = instance.element.terminal_names();
	let node_indices = self.update_node_index_map(terminal_names);
	let element_name = instance.name.to_string();
	let element_index = self.update_element_index_map(element_name);
	self.add_element_stamp(&instance, node_indices, element_index);
	self.instances.push(instance);
    }
}

impl std::fmt::Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	for instance in &self.instances {
            write!(f, "{instance}\n")?;
	}
	writeln!(f, "{}", self.node_index_map)?;
	writeln!(f, "{}", self.element_index_map)
    }
}
