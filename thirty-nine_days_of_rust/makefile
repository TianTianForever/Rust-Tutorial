ARCH? = x86_64 linux
OBJECTS = thirty-nine_days_of_rust rc_and_refcell
C = rustc
.PHONY: clean

all: $(OBJECTS)

thirty-nine_days_of_rust: thirty-nine_days_of_rust.rs
	$(C) $<  && ./$@

rc_and_refcell: rc_and_refcell.rs
	$(C) $<  && ./$@

clean:
	-rm -rf $(OBJECTS)
