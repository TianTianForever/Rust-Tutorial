ARCH? = x86_64 linux
C = rustc
.PHONY: clean
OBJS = forty-eight_days_of_rust change_data

run: $(OBJS)

forty-eight_days_of_rust: forty-eight_days_of_rust.rs
	$(C) $<  && ./$@

change_data: change_data.rs
	$(C) $< && ./$@

clean:
	-rm -rf $(OBJS)

