DAYS=d1 d2 d3 d4 d5 d6 d7 d8 d9 d10

all: ${DAYS}

${DAYS} :
	cargo build --release --bin $@
	cp target/release/$@ $@
	chmod u+x $@

.PHONY: all $(MAKECMDGOALS)
