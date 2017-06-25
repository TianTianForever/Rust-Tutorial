ARCH? = x86_64 linux
C = rustc
OBJECTS = twenty-seven_days_of_rust

.PHONY: clean

run: $(OBJECTS)

twenty-seven_days_of_rust: twenty-seven_days_of_rust.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)
