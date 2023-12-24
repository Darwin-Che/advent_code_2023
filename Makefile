DAYS=d1 d2 d3 d4 d5 d6 d7 d7p2 d8 d9 d10 d11 d12 d13 d14 d15 d16 d21 d21p2 d22 d23 d23p2

all: ${DAYS}

${DAYS} :
	cargo build --release --bin $@
	cp target/release/$@ $@
	chmod u+x $@

.PHONY: all $(MAKECMDGOALS)
