.PHONY: all install clean benchmark test

all:
	echo "make"

test:
	cd tests && python test_smoke.py $(VERBOSE)

test-verbose: VERBOSE = "-v"
test-verbose: test

setup:
	uv sync --all-groups

install:
	uv run maturin develop

clean:
	rm -rf target *.egg-info dist build */__pycache__

benchmark:
	uv run --directory tests python test_smoke.py
