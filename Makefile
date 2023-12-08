DAYS=d2

all: ${DAYS}

${DAYS} :
	cargo build --release --bin $@
	cp target/release/$@ $@
	chmod u+x $@

.PHONY: all $(MAKECMDGOALS)
