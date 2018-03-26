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


## Benchmark Result
write:
```
## benchmarker:         release 4.0.1 (for python)
## python version:      3.6.4
## python compiler:     GCC 4.2.1 Compatible Apple LLVM 9.0.0 (clang-900.0.39.2)
## python platform:     Darwin-16.7.0-x86_64-i386-64bit
## python executable:   .virtualenvs/py364/bin/python
## cpu model:           Intel(R) Core(TM) i5-5257U CPU @ 2.70GHz
## parameters:          loop=10000, cycle=1, extra=0

##                                            real    (total    = user    + sys)
std.writerows                               2.6791    1.5600    0.6100    0.9500
std.writerow                                2.6678    1.5300    0.6100    0.9200
unicodecsv.writerows                        2.5498    1.4500    0.5700    0.8800
unicodecsv.writerow                         2.5701    1.4400    0.5700    0.8700
fcsv.writerow                               0.6599    0.6300    0.2100    0.4200
fcsv.writerows                              0.6217    0.5900    0.1900    0.4000

## Ranking                                    real
fcsv.writerows                              0.6217  (100.0) ********************
fcsv.writerow                               0.6599  ( 94.2) *******************
unicodecsv.writerows                        2.5498  ( 24.4) *****
unicodecsv.writerow                         2.5701  ( 24.2) *****
std.writerow                                2.6678  ( 23.3) *****
std.writerows                               2.6791  ( 23.2) *****

## Matrix                                     real    [01]    [02]    [03]    [04]    [05]    [06]
[01] fcsv.writerows                         0.6217   100.0   106.2   410.2   413.4   429.2   431.0
[02] fcsv.writerow                          0.6599    94.2   100.0   386.4   389.5   404.3   406.0
[03] unicodecsv.writerows                   2.5498    24.4    25.9   100.0   100.8   104.6   105.1
[04] unicodecsv.writerow                    2.5701    24.2    25.7    99.2   100.0   103.8   104.2
[05] std.writerow                           2.6678    23.3    24.7    95.6    96.3   100.0   100.4
[06] std.writerows                          2.6791    23.2    24.6    95.2    95.9    99.6   100.0
```

read:
```
## benchmarker:         release 4.0.1 (for python)
## python version:      3.6.4
## python compiler:     GCC 4.2.1 Compatible Apple LLVM 9.0.0 (clang-900.0.39.2)
## python platform:     Darwin-16.7.0-x86_64-i386-64bit
## python executable:   .virtualenvs/py364/bin/python
## cpu model:           Intel(R) Core(TM) i5-5257U CPU @ 2.70GHz
## parameters:          loop=10, cycle=1, extra=0

##                                            real    (total    = user    + sys)
std.reader.10m                              2.1387    2.1300    2.0700    0.0600
unicodecsv.reader.10m                       2.1993    2.1800    2.1200    0.0600
pandas.reader.10m                           2.6152    2.5900    2.3300    0.2600
fcsv.reader.10m                             1.9403    1.9300    1.8800    0.0500

## Ranking                                    real
fcsv.reader.10m                             1.9403  (100.0) ********************
std.reader.10m                              2.1387  ( 90.7) ******************
unicodecsv.reader.10m                       2.1993  ( 88.2) ******************
pandas.reader.10m                           2.6152  ( 74.2) ***************

## Matrix                                     real    [01]    [02]    [03]    [04]
[01] fcsv.reader.10m                        1.9403   100.0   110.2   113.3   134.8
[02] std.reader.10m                         2.1387    90.7   100.0   102.8   122.3
[03] unicodecsv.reader.10m                  2.1993    88.2    97.2   100.0   118.9
[04] pandas.reader.10m                      2.6152    74.2    81.8    84.1   100.0

## benchmarker:         release 4.0.1 (for python)
## python version:      3.6.4
## python compiler:     GCC 4.2.1 Compatible Apple LLVM 9.0.0 (clang-900.0.39.2)
## python platform:     Darwin-16.7.0-x86_64-i386-64bit
## python executable:   .virtualenvs/py364/bin/python
## cpu model:           Intel(R) Core(TM) i5-5257U CPU @ 2.70GHz
## parameters:          loop=10, cycle=1, extra=0

##                                            real    (total    = user    + sys)
std.reader.100m                            19.6099   19.4700   18.9900    0.4800
unicodecsv.reader.100m                     21.4691   21.3800   20.7600    0.6200
pandas.reader.100m                         23.8532   23.7300   22.4400    1.2900
fcsv.reader.100m                           18.9651   18.9000   18.4900    0.4100

## Ranking                                    real
fcsv.reader.100m                           18.9651  (100.0) ********************
std.reader.100m                            19.6099  ( 96.7) *******************
unicodecsv.reader.100m                     21.4691  ( 88.3) ******************
pandas.reader.100m                         23.8532  ( 79.5) ****************

## Matrix                                     real    [01]    [02]    [03]    [04]
[01] fcsv.reader.100m                      18.9651   100.0   103.4   113.2   125.8
[02] std.reader.100m                       19.6099    96.7   100.0   109.5   121.6
[03] unicodecsv.reader.100m                21.4691    88.3    91.3   100.0   111.1
[04] pandas.reader.100m                    23.8532    79.5    82.2    90.0   100.0
```

