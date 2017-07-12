ARCH? = x86_64 linux

C = rustc

OBJECTS = forty-two_days_of_rust concurrency

.PHONY: clean

run: $(OBJECTS)

forty-two_days_of_rust: forty-two_days_of_rust.rs
	$(C) $< && ./$@

concurrency: concurrency.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)	
