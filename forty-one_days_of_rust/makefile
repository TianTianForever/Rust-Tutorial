ARCH? = x86_64

OBJECTS = forty-one_days_of_rust reference_cycle

C = rustc

.PHONY: clean

run: $(OBJECTS)

forty-one_days_of_rust: forty-one_days_of_rust.rs
	$(C) $< && ./$@

reference_cycle: reference_cycle.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)
