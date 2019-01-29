fn main() {
	let _simple_int_8: i8 = 12;
	let _simple_char: char = 'a';

	let mut _var_mutable_int8: i8 = 16;
	let test_underscore: i64 = 1_000;
    println!("Hello, world! de moi");

	/*
	* je copie colle des arguments
	*/

	// la le test underscore
	println!("test underscore {:?}", test_underscore);

	// Special formatting can be specified after a `:`.
	println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

	// je teste en hexa a laveugle
	println!("{} of {:X} people know binary, the other half doesn't", 1, 12);

	// You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    _var_mutable_int8 = 13;

    // cette fois c'est moi qui teste
    println!("{} {toto} {t} {d} et la normalement tout ce qui se trouve entre accolade 
    	est remplacé par les argument en var arg ", 31, toto=12, t=_simple_char, d=_var_mutable_int8);

}
