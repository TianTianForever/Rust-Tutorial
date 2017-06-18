ARCH? = x86_64
LINKS = --extern
R = rustc

.PHONY: run clean
OBJECTS = twenty-one_days_of_rust maps

run: $(OBJECTS)

twenty-one_days_of_rust: twenty-one_days_of_rust.rs
	$(R) $< && ./$@

maps: maps.rs
	$(R) $< && ./$@

clean:
	-rm -rf $(OBJECTS)
