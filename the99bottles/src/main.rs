
fn bottles() {
	let mut bootles_var: i8 = 99;
	println!("in bottle function");
	println!("nombre de bouteill {:?}", bootles_var);
	// bootles_var -= 1;

	loop {
		println!("{:?} bottles of beer on the wall, {:?} bottles of beer.", bootles_var, bootles_var);
		bootles_var -= 1;
		if bootles_var > 0 {
			println!("Take one down and pass it around, {:?} bottles of beer on the wall.", bootles_var);
		}
		else {
			println!("Take one down and pass it around, no more bottles of beer on the wall.");
			break;
		}
	}
	println!("No more bottles of beer on the wall, no more bottles of beer.");
	println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
}

fn main() {
    println!("Hello, world!");
    bottles();
}
