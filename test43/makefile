all: libhello.rlib executable.rs
	rustc  executable.rs --extern hello=libhello.rlib && ./executable

libhello.rlib: hello.rs
	rustc --crate-type=lib hello.rs
clean:
	rm -rf libhello.rlib executable
