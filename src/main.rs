use csuperlu::{sparse_matrix::SparseMatrix, simple_driver::SimpleSystem, c::{stat::CSuperluStat, options::ColumnPermPolicy}, dense::DenseMatrix};

fn main() {

    let mut a = SparseMatrix::new(4, 4);
    a.set_value(0, 0, 1.0);
    a.set_value(1, 1, 1.0);
    a.set_value(2, 2, 1.0);
    a.set_value(3, 3, 1.0);

    let a = a.compressed_column_format();
    let b = vec![1.0; 4];
    let b = DenseMatrix::from_vectors(4, 1, b);
    
    let system = SimpleSystem {
	a,
	b,
    };

    let mut stat = CSuperluStat::new();
    
    let solution = system.solve(&mut stat, ColumnPermPolicy::ColAMD).unwrap();

    solution.x.print("Solution");
    
}
