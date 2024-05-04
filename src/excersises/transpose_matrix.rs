fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
	let mut transpose_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
	for row_index in 0..3{
		for col_index in 0..3{
			tranposed_matrix[row_index][col_index] = matrix[col_index][row_index];
		}
	}
	tranpose_matrix
}


#[test]
fn test_transpose(){
	let matrix = [
		[101,102,103],
		[201,202,203],
		[301, 302, 303],
	];
	
	let transposed = transpose(matrix);
	assert_eq!(
		transposed,
		[
			[101, 102, 103],
			[102, 202, 302],
			[103, 203, 303],
		]
	);
}


fn main(){
	let matrix = [
		[101, 102, 103],
		[201, 202, 203],
		[301, 302, 303],
	];

	println!("Matrix: {:#?}", matrix);
	let transposed = transpose(matrix);
	println!("Transposed: {:#?}", transnposed);
}
