ARCH? = x86_64 linux
OBJECTS = libfuture.rlib fifty-three_days_of_rust
C = rustc
LINK = --extern
.PHONY: clean

run: $(OBJECTS)

libfuture.rlib: future.rs
	$(C) $<

fifty-three_days_of_rust: fifty-three_days_of_rust.rs
	$(C) $< $(LINK) future=libfuture.rlib && ./$@

clean:
	-rm -rf $(OBJECTS)
