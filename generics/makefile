OBJS = generics functions test test2 implementations \
       impl1 impl2 traits traits_test1

all: generics function test test2 implementations \
     impl1 impl2 traits traits_test1

generics: generics.rs
	rustc generics.rs && ./generics

function: functions.rs
	rustc functions.rs && ./functions

test: test.rs
	rustc test.rs && ./test

test2: test2.rs
	rustc test2.rs && ./test2

implementations: implementations.rs
	rustc implementations.rs && ./implementations

impl1: impl1.rs	
	rustc impl1.rs && ./impl1

impl2: impl2.rs
	rustc impl2.rs && ./impl2

traits: traits.rs
	rustc traits.rs && ./traits

traits_test1: traits_test1.rs
	rustc traits_test1.rs && ./traits_test1

clean:
	rm -rf $(OBJS)
