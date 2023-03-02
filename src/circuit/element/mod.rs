pub mod passive;
pub mod active;

use super::{MnaMatrix, MnaRhs};

pub trait Element: std::fmt::Display {
    fn terminal_names(&self) -> Vec<String>;
    fn add_stamp(&self,
		 mna_matrix: &mut MnaMatrix,
		 mna_rhs: &mut MnaRhs,
		 node_indices: Vec<usize>,
		 element_index: usize,
		 group2: bool);
}

