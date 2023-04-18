use std::env;


fn main () {
	let args: Vec<String> = env::args().collect();
	let filepath = &args[1];
	println!("Filepath: '{filepath}'");

	for (idx, arg) in args[2..].iter().enumerate() {
		print!("\tArg #{idx}: '{arg}'\n");
	}

}
