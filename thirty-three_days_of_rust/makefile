ARCH? = x86_64 linux
C = rustc
OBJECTS = thirty-three_days_of_rust
.PHONY: clean

run: $(OBJECTS)

thirty-three_days_of_rust: thirty-three_days_of_rust.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)

