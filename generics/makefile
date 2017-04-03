OBJS = generics functions test test2
all: generics function test test2

generics: generics.rs
	rustc generics.rs && ./generics
function: functions.rs
	rustc functions.rs && ./functions
test: test.rs
	rustc test.rs && ./test
test2: test2.rs
	rustc test2.rs && ./test2
clean:
	rm -rf $(OBJS)
