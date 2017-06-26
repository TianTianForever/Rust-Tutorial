ARCH? = x86_64 linux
C = rustc
OBJECTS = twenty-eight_days_of_rust my_errors

.PHONY: clean

run: $(OBJECTS)

twenty-eight_days_of_rust: twenty-eight_days_of_rust.rs
	$(C) $< && ./$@

my_errors: my_errors.rs
	$(C) $< && ./$@

clean:
	rm -rf $(OBJECTS)
