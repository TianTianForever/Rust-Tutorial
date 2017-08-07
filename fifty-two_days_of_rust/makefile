ARCH? = x86_64 linux
C = rustc
OBJECTS = fifty-two_days_of_rust

.PHONY: clean

run: $(OBJECTS)

fifty-two_days_of_rust: fifty-two_days_of_rust.rs
	$(C) $<  && ./$@

clean:
	-rm -rf $(OBJECTS)
