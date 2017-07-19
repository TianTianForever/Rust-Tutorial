ARCH? = x86_64 linux
OBJS = libobjects.rlib forty-seven_days_of_rust
C = rustc
LINK = --extern
.PHONY: clean

run: $(OBJS)

libobjects.rlib: objects.rs
	$(C) $<

forty-seven_days_of_rust: forty-seven_days_of_rust.rs
	$(C) $<  $(LINK) objects=libobjects.rlib && ./$@


clean:
	-rm -rf $(OBJS)
