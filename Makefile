.PHONY: all install clean benchmark test

all:
	echo "make"

test:
	cd tests && python test_smoke.py $(VERBOSE)

test-verbose: VERBOSE = "-v"
test-verbose: test

install:
	python setup.py install

install-pip:
	pip install --upgrade . $(VERBOSE)

install-pip-verbose: VERBOSE = "-v"
install-pip-verbose: install-pip

clean:
	python setup.py clean
	rm -rf *.egg-info dist build */__pycache__

benchmark:
	cd benchmarks && python benchmark.py
