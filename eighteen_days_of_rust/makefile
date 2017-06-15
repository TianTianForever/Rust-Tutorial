ARCH?= linux
OBJECTS = eighteen_days_of_rust any_and_iterators
R = rustc
LINKS = extern

.PHONY: run clean

run: $(OBJECTS)

eighteen_days_of_rust: eighteen_days_of_rust.rs
	$(R) $< && ./$@

any_and_iterators: any_and_iterators.rs
	$(R) $< && ./$@
clean:
	-rm -rf $(OBJECTS)
