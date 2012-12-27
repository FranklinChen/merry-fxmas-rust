RUSTC = rustc

RESULT = ./main

all:	$(RESULT)

$(RESULT):	main.rs
	$(RUSTC) $<

run:	$(RESULT)
	$(RESULT)

.PHONY:	all run
