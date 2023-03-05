use csuperlu::{sparse_matrix::SparseMatrix, c::{options::ColumnPermPolicy, stat::CSuperluStat}, dense::DenseMatrix, simple_driver::SimpleSystem};

pub fn transpose(mut a: SparseMatrix<f64>) -> SparseMatrix<f64> {
    // This does not modify in place yet -- todo
    let mut transposed = SparseMatrix::new();

    //    
    for ((row, col), value) in a.values().iter() {
	transposed.set_value(*col, *row, *value);
    }
    transposed
}

/// Assumes that the two matrices use the same row indices, even if heights disagree. The
/// smaller matrix is assumed to have zero rows up to the size of the larger matrix. The
/// matrices are concatenated with horizontal padding, which adds h_pad all-zero columns between
/// a and b  
pub fn concat_horizontal(
    mut a: SparseMatrix<f64>,
    b: &SparseMatrix<f64>,
    h_pad: usize
) -> SparseMatrix<f64> {
    let offset = a.num_cols() + h_pad;
    for ((row, col), value) in b.values().iter() {
	a.set_value(*row, offset + *col, *value);
    }
    a
}

pub fn concat_vertical(
    mut a: SparseMatrix<f64>,
    b: &SparseMatrix<f64>,
    v_pad: usize,
) -> SparseMatrix<f64> {
    let offset = a.num_rows() + v_pad;
    for ((row, col), value) in b.values().iter() {
	a.set_value(offset + *row, *col, *value);
    }
    a
}

pub fn neg(mat: SparseMatrix<f64>) -> SparseMatrix<f64> {
    let mut new_mat = SparseMatrix::<f64>::new();
    for ((row, col), value) in mat.values().iter() {
	new_mat.set_value(*row, *col, *value);
    }
    new_mat
}

pub fn solve(a: SparseMatrix<f64>, b: Vec<f64>) -> Vec<f64> {
    if a.num_rows() != b.len() {
	panic!("Cannot solve system; incompatible dimensions");
    }
    let a = a.compressed_column_format();
    a.print("a");
    let b = DenseMatrix::from_vectors(b.len(), 1, b);
    let system = SimpleSystem {
	a,
	b,
    };
    let mut stat = CSuperluStat::new();
    let mut solution = system.solve(&mut stat, ColumnPermPolicy::ColAMD)
	.expect("Failed to solve system");
    solution.x.column_major_values().to_vec()
}
