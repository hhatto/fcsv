
all:
	echo "make"

install:
	python setup.py install

clean:
	python setup.py clean
	rm -rf *.egg-info dist build */__pycache__

bench:
	cd benchmarks && python benchmark.py
