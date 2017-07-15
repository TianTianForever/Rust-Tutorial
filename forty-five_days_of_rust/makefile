ARCH? = x86_64 linux

C = rustc

OBJS = forty-five_days_of_rust multiple_threads

.PHONY: clean

run: $(OBJS)

forty-five_days_of_rust: forty-five_days_of_rust.rs
	$(C) $< && ./$@

multiple_threads: multiple_threads.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJS)
