ARCH? = x86_64 linux
C = rustc
OBJECTS = thirty-one_days_of_rust

.PHONY: clean

run: $(OBJECTS)

thirty-one_days_of_rust: thirty-one_days_of_rust.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)

