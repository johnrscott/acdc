use csuperlu::{sparse_matrix::SparseMatrix, c::value_type::ValueType};

pub fn transpose<P: ValueType<P>>(mut a: SparseMatrix<P>) -> SparseMatrix<P> {
    // This does not modify in place yet -- todo
    let mut transposed = SparseMatrix::new(a.num_rows(),
					   a.num_cols());

    //    
    for ((row, col), value) in a.values().iter() {
	transposed.set_value(*col, *row, *value);
    }
    transposed
}

pub fn concat_horizontal<P: ValueType<P>>(mut a: SparseMatrix<P>, b: &SparseMatrix<P>) -> SparseMatrix<P> {
    if a.num_rows() != b.num_rows() {
	panic!("Cannot horizontally concatenate matrices of different heights");
    }
    let num_cols_a = a.num_cols();
    for ((row, col), value) in b.values().iter() {
	a.set_value(*row, num_cols_a + *col, *value);
    }
    a
}

pub fn concat_vertical<P: ValueType<P>>(mut a: SparseMatrix<P>, b: &SparseMatrix<P>) -> SparseMatrix<P> {
    if a.num_cols() != b.num_cols() {
	panic!("Cannot horizontally concatenate matrices of different widths");
    }
    let num_rows_a = a.num_rows();
    for ((row, col), value) in b.values().iter() {
	a.set_value(num_rows_a + *row, *col, *value);
    }
    a
}

fn neg<P: ValueType<P>>(mat: SparseMatrix<P>) -> SparseMatrix<P> {
    let mut new_mat = SparseMatrix::<P>::new(mat.num_rows(), mat.num_cols());
    for ((row, col), value) in mat.values().iter() {
	new_mat.set_value(*row, *col, *value);
    }
    new_mat
}
