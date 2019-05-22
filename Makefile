
SOURCES := $(wildcard src/*.rs)

fmt: $(SOURCES)
	rustfmt $(SOURCES)
