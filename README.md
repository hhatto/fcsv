# fcsv [![](https://travis-ci.org/hhatto/fcsv.svg?branch=master)](https://travis-ci.org/hhatto/fcsv)
yet another python csv module


## Requirements

* [setuptools-rust](https://github.com/PyO3/setuptools-rust)
* Nightly Rust

```
$ pip install setuptools_rust
$ rustup default nightly
```


## Installation

```
$ pip install --upgrade git+https://github.com/hhatto/fcsv
```


## Benchmark Results

### write

```
## benchmarker:         release 4.0.1 (for python)
## python version:      3.9.13
## python compiler:     Clang 13.1.6 (clang-1316.0.21.2)
## python platform:     macOS-12.4-arm64-arm-64bit
## python executable:   /Users/hattorihideo/.virtualenvs/py396/bin/python
## cpu model:           Apple M1
## parameters:          loop=10000, cycle=1, extra=0

##                                            real    (total    = user    + sys)
std.writerows                               0.6936    0.5900    0.0600    0.5300
std.writerow                                0.8086    0.5900    0.0700    0.5200
unicodecsv.writerows                        0.5954    0.5600    0.0600    0.5000
unicodecsv.writerow                         0.5749    0.5100    0.0600    0.4500
fcsv.writerow                               0.3038    0.2600    0.0300    0.2300
fcsv.writerows                              0.2070    0.2100    0.0200    0.1900

## Ranking                                    real
fcsv.writerows                              0.2070  (100.0) ********************
fcsv.writerow                               0.3038  ( 68.1) **************
unicodecsv.writerow                         0.5749  ( 36.0) *******
unicodecsv.writerows                        0.5954  ( 34.8) *******
std.writerows                               0.6936  ( 29.8) ******
std.writerow                                0.8086  ( 25.6) *****

## Matrix                                     real    [01]    [02]    [03]    [04]    [05]    [06]
[01] fcsv.writerows                         0.2070   100.0   146.8   277.7   287.6   335.0   390.6
[02] fcsv.writerow                          0.3038    68.1   100.0   189.2   196.0   228.3   266.1
[03] unicodecsv.writerow                    0.5749    36.0    52.8   100.0   103.6   120.6   140.6
[04] unicodecsv.writerows                   0.5954    34.8    51.0    96.6   100.0   116.5   135.8
[05] std.writerows                          0.6936    29.8    43.8    82.9    85.8   100.0   116.6
[06] std.writerow                           0.8086    25.6    37.6    71.1    73.6    85.8   100.0
```

### read

```
## benchmarker:         release 4.0.1 (for python)
## python version:      3.9.13
## python compiler:     Clang 13.1.6 (clang-1316.0.21.2)
## python platform:     macOS-12.4-arm64-arm-64bit
## python executable:   /Users/hattorihideo/.virtualenvs/py396/bin/python
## cpu model:           Apple M1
## parameters:          loop=10, cycle=1, extra=0

##                                            real    (total    = user    + sys)
std.reader.10m                              1.0061    1.0000    0.9900    0.0100
unicodecsv.reader.10m                       0.8523    0.8600    0.8400    0.0200
pandas.reader.10m                           0.9625    0.9500    0.8700    0.0800
fcsv.reader.10m                             0.7306    0.7300    0.7200    0.0100

## Ranking                                    real
fcsv.reader.10m                             0.7306  (100.0) ********************
unicodecsv.reader.10m                       0.8523  ( 85.7) *****************
pandas.reader.10m                           0.9625  ( 75.9) ***************
std.reader.10m                              1.0061  ( 72.6) ***************

## Matrix                                     real    [01]    [02]    [03]    [04]
[01] fcsv.reader.10m                        0.7306   100.0   116.7   131.8   137.7
[02] unicodecsv.reader.10m                  0.8523    85.7   100.0   112.9   118.0
[03] pandas.reader.10m                      0.9625    75.9    88.5   100.0   104.5
[04] std.reader.10m                         1.0061    72.6    84.7    95.7   100.0

## benchmarker:         release 4.0.1 (for python)
## python version:      3.9.13
## python compiler:     Clang 13.1.6 (clang-1316.0.21.2)
## python platform:     macOS-12.4-arm64-arm-64bit
## python executable:   /Users/hattorihideo/.virtualenvs/py396/bin/python
## cpu model:           Apple M1
## parameters:          loop=10, cycle=1, extra=0

##                                            real    (total    = user    + sys)
std.reader.100m                             8.5139    8.4400    8.3000    0.1400
unicodecsv.reader.100m                      8.6436    8.6100    8.4400    0.1700
pandas.reader.100m                          8.6361    8.5800    8.0200    0.5600
fcsv.reader.100m                            7.4359    7.3800    7.2500    0.1300

## Ranking                                    real
fcsv.reader.100m                            7.4359  (100.0) ********************
std.reader.100m                             8.5139  ( 87.3) *****************
pandas.reader.100m                          8.6361  ( 86.1) *****************
unicodecsv.reader.100m                      8.6436  ( 86.0) *****************

## Matrix                                     real    [01]    [02]    [03]    [04]
[01] fcsv.reader.100m                       7.4359   100.0   114.5   116.1   116.2
[02] std.reader.100m                        8.5139    87.3   100.0   101.4   101.5
[03] pandas.reader.100m                     8.6361    86.1    98.6   100.0   100.1
[04] unicodecsv.reader.100m                 8.6436    86.0    98.5    99.9   100.0
```

## For Developer

setup environment:

```
$ make setup
```

build & install:
```
$ make install
```
