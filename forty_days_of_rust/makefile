ARCH? = x86_64 linux

OBJECTS = forty_days_of_rust smart_pointer closure

C = rustc

.PHONY: clean

all: $(OBJECTS)

forty_days_of_rust: forty_days_of_rust.rs
	$(C) $< && ./$@

smart_pointer: smart_pointer.rs
	$(C) $< && ./$@

closure: closure.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJECTS)
