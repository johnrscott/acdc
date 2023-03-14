//! Sparse matrix utilities
//!
//! This file acts as an interface between this application and
//! csuperlu 
//!

use csuperlu::{sparse_matrix::SparseMat, dense::DenseMatrix, simple_driver::{SimpleSystem, SimpleSolution}, c::{stat::CSuperluStat, options::ColumnPermPolicy}};

/// Assumes the matrix is square
pub fn plus_equals(mat: &mut SparseMat<f64>, row: usize, col: usize, val: f64) {
    let old_val = mat.get_unbounded(row, col);
    mat.insert_unbounded(row, col, old_val + val);
}

/// Assumes that the two matrices use the same row indices, even if heights disagree. The
/// smaller matrix is assumed to have zero rows up to the size of the larger matrix. The
/// matrices are concatenated with horizontal padding, which adds h_pad all-zero columns between
/// a and b  
pub fn concat_horizontal(mut a: SparseMat<f64>, b: &SparseMat<f64>) -> SparseMat<f64> {
    if a.num_rows() != b.num_rows() {
        panic!("Cannot concatenate matrices horizontally with different numbers of rows");
    }
    let a_cols = a.num_cols();
    for ((row, col), value) in b.non_zero_vals().iter() {
        a.insert_unbounded(*row, a_cols + *col, *value);
    }
    a.resize_cols(a_cols + b.num_cols());
    a
}

pub fn concat_vertical(mut a: SparseMat<f64>, b: &SparseMat<f64>) -> SparseMat<f64> {
    if a.num_cols() != b.num_cols() {
        panic!(
            "Cannot concatenate matrices vertically with different numbers of columns {} and {}",
            a.num_cols(),
            b.num_cols()
        );
    }
    let a_rows = a.num_rows();
    for ((row, col), value) in b.non_zero_vals().iter() {
        a.insert_unbounded(a_rows + *row, *col, *value);
    }
    a.resize_rows(a_rows + b.num_rows());
    a
}

pub fn solve(a: SparseMat<f64>, b: Vec<f64>) -> Vec<f64> {
    if a.num_rows() != b.len() {
        panic!("Cannot solve system; incompatible dimensions");
    }
    let a = a.compressed_column_format();
    a.print("a");
    let b = DenseMatrix::from_vectors(b.len(), 1, b);
    let system = SimpleSystem { a, b };
    let mut stat = CSuperluStat::new();
    let SimpleSolution {
	mut x,
	..
    }= system.solve(&mut stat, ColumnPermPolicy::ColAMD)
        .expect("Failed to solve system");

    x.column_major_values().to_vec()
}
