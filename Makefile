.PHONY: all install clean benchmark test

all:
	echo "make"

test:
	uv run --directory tests python test_smoke.py

test-verbose: VERBOSE = "-v"
test-verbose: test

setup:
	uv sync --all-groups

install:
	uv run maturin develop

clean:
	rm -rf target *.egg-info dist build */__pycache__

benchmark:
	uv run --directory benchmarks python benchmark.py
