from benchmarker import Benchmarker
import os
import tempfile
import unicodecsv
import csv
import fcsv
import pandas as pd

NUM = 10 * 1000
source = [["abc", "def", "ghi"],
          ["jkl", "あいう", "opq"],
          ["vvv", "v1", "v2"]]
source = (("abc", "def", "ghi"),
          ("jkl", "あいう", "opq"),
          ("vvv", "v1", "v2"))

with Benchmarker(NUM, width=40) as bench:

    @bench("std.writerows")
    def b_std_writerows(bm):
        for i in bm:
            with open('b_std.csv', 'w') as out:
                writer = csv.writer(out, quoting=csv.QUOTE_NONNUMERIC)
                writer.writerows(source)

    @bench("std.writerow")
    def b_std_writerow(bm):
        for i in bm:
            with open('b_std.csv', 'w') as out:
                writer = csv.writer(out, quoting=csv.QUOTE_NONNUMERIC)
                writer.writerow(source[0])
                writer.writerow(source[1])
                writer.writerow(source[2])

    @bench("unicodecsv.writerows")
    def b_unicodecsv_writerows(bm):
        for i in bm:
            with open('b_unicodecsv.csv', 'wb') as out:
                writer = unicodecsv.writer(out, quoting=csv.QUOTE_NONNUMERIC)
                writer.writerows(source)

    @bench("unicodecsv.writerow")
    def b_unicodecsv_writerow(bm):
        for i in bm:
            with open('b_unicodecsv.csv', 'wb') as out:
                writer = unicodecsv.writer(out, quoting=csv.QUOTE_NONNUMERIC)
                writer.writerow(source[0])
                writer.writerow(source[1])
                writer.writerow(source[2])

    @bench("fcsv.writerow")
    def b_fcsv_writerow(bm):
        for i in bm:
            writer = fcsv.Writer('b_fcsv.csv')  # , quoting=csv.QUOTE_NONNUMERIC)
            writer.writerow(source[0])
            writer.writerow(source[1])
            writer.writerow(source[2])

    @bench("fcsv.writerows")
    def b_fcsv_writerows(bm):
        for i in bm:
            writer = fcsv.Writer('b_fcsv.csv')  # , quoting=csv.QUOTE_NONNUMERIC)
            writer.writerows(source)


NUM = 10
with Benchmarker(NUM, width=40) as bench:

    @bench("std.reader.10m")
    def b_std_reader(bm):
        for i in bm:
            with open('b_reader.csv') as fl:
                reader = csv.reader(fl)
                for row in reader:
                    _ = row

    @bench("unicodecsv.reader.10m")
    def b_unicodecsv_reader(bm):
        for i in bm:
            with open('b_reader.csv', 'rb') as out:
                reader = unicodecsv.reader(out, 'excel')
                for row in reader:
                    _ = row

    @bench("pandas.reader.10m")
    def b_pandas_reader(bm):
        for i in bm:
            for row in pd.read_csv('b_reader.csv'):
                _ = row

    @bench("fcsv.reader.10m")
    def b_fcsv_reader(bm):
        for i in bm:
            reader = fcsv.Reader('b_reader.csv')  # , quoting=csv.QUOTE_NONNUMERIC)
            for row in reader:
                _ = row


large_csv_file = tempfile.NamedTemporaryFile('w')
with open('b_reader.csv') as f:
    s = f.read()
for i in range(10):
    large_csv_file.write(s)
large_csv_file.flush()
large_csv_filename = large_csv_file.name
print("%s" % large_csv_filename)
with Benchmarker(NUM, width=40) as bench:

    @bench("std.reader.100m")
    def b_std_reader(bm):
        for i in bm:
            with open(large_csv_file.name) as fl:
                reader = csv.reader(fl)
                for row in reader:
                    _ = row

    @bench("unicodecsv.reader.100m")
    def b_unicodecsv_reader(bm):
        for i in bm:
            with open(large_csv_file.name, 'rb') as out:
                reader = unicodecsv.reader(out, 'excel')
                for row in reader:
                    _ = row

    @bench("pandas.reader.100m")
    def b_pandas_reader(bm):
        for i in bm:
            for row in pd.read_csv(large_csv_file.name):
                _ = row

    @bench("fcsv.reader.100m")
    def b_fcsv_reader(bm):
        for i in bm:
            reader = fcsv.Reader(large_csv_file.name)  # , quoting=csv.QUOTE_NONNUMERIC)
            for row in reader:
                _ = row

large_csv_file.close()
try:
    os.remove(large_csv_file.name)
except:
    # already removed
    pass
