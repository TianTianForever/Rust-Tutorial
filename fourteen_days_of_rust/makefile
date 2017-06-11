ARCH?=x86_64
OBJECTS =  sub_add calculate fourteen_days_of_rust
RR = rustc

.PHONY: all clean

all: $(OBJECTS)

calculate: calculate.rs
	$(RR) $< && ./$@

sub_add: sub_add.rs
	$(RR) $< && ./$@

fourteen_days_of_rust: fourteen_days_of_rust.rs
	$(RR) $< && ./$@

clean:
	-rm -rf $(OBJECTS)

