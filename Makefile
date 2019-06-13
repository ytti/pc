
SOURCES := $(wildcard src/*/*.rs src/*.rs)

fmt: $(SOURCES)
	rustfmt $(SOURCES)

test:
	./tests/test_config.sh
