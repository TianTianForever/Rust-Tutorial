ARCH? = linux x86_64
OBJECTS = twenty-two_days_of_rust errors
C = rustc

.PHONY: run clean

run: $(OBJECTS)

twenty-two_days_of_rust: twenty-two_days_of_rust.rs
	$(C) $< && ./$@

errors: errors.rs
	$(C) $< && ./$@
clean:
	-rm -rf $(OBJECTS)
