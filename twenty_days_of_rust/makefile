ARCH? = x86_64 linux
LINKS = --extern
R = rustc

.PHONY: run clean

OBJECTS = twenty_days_of_rust my_map

run: $(OBJECTS)

twenty_days_of_rust: twenty_days_of_rust.rs
	$(R) $< && ./$@

my_map: my_map.rs
	$(R) $< && ./$@

clean:
	-rm -rf $(OBJECTS)
