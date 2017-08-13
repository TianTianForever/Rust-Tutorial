ARCH? = x86_64 linux
TARGETS = libwrapper.rlib fifty-five_days_of_rust
C = rustc
LINK = --extern
.PHONY: clean

run: $(TARGETS)

libwrapper.rlib: wrapper.rs
	$(C) $<

fifty-five_days_of_rust: fifty-five_days_of_rust.rs
	$(C) $< $(LINK) wrapper=libwrapper.rlib && ./$@

clean:
	-rm -rf $(TARGETS)
