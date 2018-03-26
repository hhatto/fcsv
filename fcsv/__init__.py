import csv
import _fcsv
from io import StringIO

def reader(csvfile, dialect='excel', **fmtparams):
    # only support file-like object and file path string
    if type(csvfile) != str:
        csvfile = csvfile.fileno()
    return _fcsv.Reader(csvfile)

#def writer(csvfile, dialect='excel', **fmtparams):
#    # only support file-like object and file path string
#    if type(csvfile) != str:
#        csvfile = csvfile.fileno()
#    return _fcsv.Writer(csvfile, dialect, **fmtparams)
writer = _fcsv.Writer

excel = csv.excel
QUOTE_MINIMAL = csv.QUOTE_MINIMAL
QUOTE_ALL = csv.QUOTE_ALL
QUOTE_NONNUMERIC = csv.QUOTE_NONNUMERIC
QUOTE_NONE = csv.QUOTE_NONE
