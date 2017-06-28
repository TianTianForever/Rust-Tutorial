ARCH? = x86_64 linux
OBJECTS = thirty_days_of_rust
C = rustc

.PHONY: clean

run: $(OBJECTS)

thirty_days_of_rust: thirty_days_of_rust.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)
