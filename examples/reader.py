import sys
import csv
import fcsv

if len(sys.argv) < 2:
    print("usage: prog CSV_FILENAME")
    sys.exit(1)
filename = sys.argv[1]


def std_read():
    with open(filename) as fp:
        reader = csv.reader(fp)
        for row in reader:
            print(row)


def fcsv_read():
    reader = fcsv.reader(filename)
    for row in reader.read():
        print(row)


std_read()
fcsv_read()
