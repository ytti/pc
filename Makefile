
SOURCES := $(wildcard src/*/*.rs src/*.rs)

fmt: $(SOURCES)
	rustfmt $(SOURCES)
