use csuperlu::{
    c::{options::ColumnPermPolicy, stat::CSuperluStat},
    dense::DenseMatrix,
    simple_driver::{SimpleSystem, SimpleSolution},
    sparse_matrix::SparseMat,
};

pub fn transpose(mut a: SparseMat<f64>) -> SparseMat<f64> {
    // This does not modify in place yet -- todo
    let mut transposed = SparseMat::empty();

    //
    for ((row, col), value) in a.non_zero_vals().iter() {
        transposed.insert_unbounded(*col, *row, *value);
    }
    transposed
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

pub fn neg(mat: SparseMat<f64>) -> SparseMat<f64> {
    let mut new_mat = SparseMat::<f64>::empty();
    for ((row, col), value) in mat.non_zero_vals().iter() {
        new_mat.insert_unbounded(*row, *col, *value);
    }
    new_mat
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
