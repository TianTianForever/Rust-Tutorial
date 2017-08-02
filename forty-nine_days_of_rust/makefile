ARCH? = x86_64 linux

OBJECTS= forty-nine_days_of_rust

C = rustc

.PHONY: clean

run: $(OBJECTS)

forty-nine_days_of_rust: forty-nine_days_of_rust.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)

