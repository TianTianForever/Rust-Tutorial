all: library.rlib executable
	./executable

library.rlib: rary.rs
	rustc --crate-type=lib rary.rs

executable: executable.rs
	rustc executable.rs --extern rary=library.rlib	

clean:
	rm -rf library.rlib executable
