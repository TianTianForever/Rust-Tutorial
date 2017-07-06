ARCH? = x86_64
OBJECTS = thirty-seven_days_of_rust SmartPointer
C = rustc

.PHONY: clean

run: $(OBJECTS)

thirty-seven_days_of_rust: thirty-seven_days_of_rust.rs
	$(C) $<  && ./$@

SmartPointer: SmartPointer.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)
