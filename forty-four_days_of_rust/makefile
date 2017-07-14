ARCH? = x86_64 linux

OBJS = forty-four_days_of_rust shared_state_concurrency
C = rustc

.PHONY: clean

run: $(OBJS)
forty-four_days_of_rust: forty-four_days_of_rust.rs
	$(C) $< && ./$@

shared_state_concurrency: shared_state_concurrency.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJS)
