ARCH? = x86_64 linux
C = rustc
OBJECTS = thirty-two_days_of_rust

.PHONY: clean

run: $(OBJECTS)

thirty-two_days_of_rust: thirty-two_days_of_rust.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)
