ARCH? = x86_64 linux
OBJECTS = forty-three_days_of_rust concurrency
C = rustc

.PHONY: clean

run: $(OBJECTS)
	
forty-three_days_of_rust: forty-three_days_of_rust.rs
	$(C) $< && ./$@

concurrency: concurrency.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS) 

