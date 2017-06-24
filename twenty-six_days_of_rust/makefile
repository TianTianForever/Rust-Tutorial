ARCH? = x86_64 linux
C = rustc
LINKS = --extern

OBJECTS = twenty-six_days_of_rust tiantian.txt forever.txt

run: $(OBJECTS)
	more tiantian.txt forever.txt

twenty-six_days_of_rust: twenty-six_days_of_rust.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)
