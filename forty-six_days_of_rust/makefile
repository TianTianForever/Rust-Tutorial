ARCH? =x86_64 linux
C = rustc
OBJS = forty-six_days_of_rust

.PHONY: clean

run: $(OBJS)
	
forty-six_days_of_rust: forty-six_days_of_rust.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJS)
