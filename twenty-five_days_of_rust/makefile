ARCH? = x86_64 linux
LINK = --extern
C = rustc

OBJECTS = liberrors.rlib twenty-five_days_of_rust a b

.PHONY: clean

run: $(OBJECTS)

liberrors.rlib: errors.rs
	$(C) $<

twenty-five_days_of_rust: twenty-five_days_of_rust.rs
	$(C) $< $(LINK) errors=liberrors.rlib && ./twenty-five_days_of_rust

a: a.txt
	more a.txt

b: b.txt
	more b.txt
clean:
	-rm -rf $(OBJECTS) a.txt b.txt

