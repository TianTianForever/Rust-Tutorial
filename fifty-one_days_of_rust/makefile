ARCH? = x86_64 linux
C = rustc
OBJS = fifty-one_days_of_rust
.PHONY: clean

run: $(OBJS)

fifty-one_days_of_rust: fifty-one_days_of_rust.rs
	$(C) $<  && ./$@

clean:
	-rm -rf $(OBJS)
