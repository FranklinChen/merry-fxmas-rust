RUSTC = rustc

RESULT = ./main

all:	$(RESULT)

$(RESULT):	main.rs
	$(RUSTC) $<

run:	$(RESULT)
	$(RESULT)

clean:
	-rm -rf main.dSYM

.PHONY:	all run clean
