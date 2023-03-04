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

pub fn concat_horizontal(mut a: SparseMatrix<f64>, b: &SparseMatrix<f64>) -> SparseMatrix<f64> {
    if a.num_rows() != b.num_rows() {
	panic!("Cannot horizontally concatenate matrices of different heights");
    }
    let num_cols_a = a.num_cols();
    for ((row, col), value) in b.values().iter() {
	a.set_value(*row, num_cols_a + *col, *value);
    }
    a
}

pub fn concat_vertical(mut a: SparseMatrix<f64>, b: &SparseMatrix<f64>) -> SparseMatrix<f64> {
    if a.num_cols() != b.num_cols() {
	panic!("Cannot horizontally concatenate matrices of different widths");
    }
    let num_rows_a = a.num_rows();
    for ((row, col), value) in b.values().iter() {
	a.set_value(num_rows_a + *row, *col, *value);
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
