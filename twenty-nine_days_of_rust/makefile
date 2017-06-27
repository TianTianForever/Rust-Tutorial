ARCH? = x86_64 linux
C = rustc

OBJECTS = twenty-nine_days_of_rust  match_reference

.PHONY: clean

run: $(OBJECTS)

twenty-nine_days_of_rust: twenty-nine_days_of_rust.rs
	$(C) $<  && ./$@

match_reference: match_reference.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)
