import os
import unittest
from tempfile import NamedTemporaryFile
import csv
import fcsv


class TestWriterSmoke(unittest.TestCase):

    def setUp(self):
        self.fo = NamedTemporaryFile("w", delete=False)
        self.fo.close()

    def tearDown(self):
        os.remove(self.fo.name)

    def test_writer_with_list(self):
        rowdata = [1, 2, -4.1, "Hello", True, False, None]
        # fcsv
        w = fcsv.writer(self.fo.name)
        w.writerow(rowdata)
        with open(self.fo.name) as f:
            ret = f.read().strip()
            self.assertEqual("1,2,-4.1,Hello,True,False,", ret)

        # std
        with open(self.fo.name, 'w') as f:
            w = csv.writer(f)
            w.writerow(rowdata)

        # validation
        with open(self.fo.name) as f:
            std_ret = f.read().strip()
            self.assertEqual(ret, std_ret)

    def test_writer_opt_dialect_excel(self):
        rowdata = [1, 2, -4.1, "Hello", True, False, None]
        # fcsv
        w = fcsv.writer(self.fo.name, dialect="excel")
        w.writerow(rowdata)
        with open(self.fo.name) as f:
            ret = f.read().strip()
        self.assertEqual("1,2,-4.1,Hello,True,False,", ret)

        # std
        with open(self.fo.name, 'w') as f:
            w = csv.writer(f, dialect="excel")
            w.writerow(rowdata)
        with open(self.fo.name) as f:
            std_ret = f.read().strip()

        # validation
        self.assertEqual(ret, std_ret)

    def test_writer_opt_dialect_and_delimiter(self):
        rowdata = [1, 2, -4.1, "Hello", True, False, None]
        # fcsv
        w = fcsv.writer(self.fo.name, dialect="excel", delimiter=";")
        w.writerow(rowdata)
        with open(self.fo.name) as f:
            ret = f.read().strip()
        self.assertEqual("1;2;-4.1;Hello;True;False;", ret)

        # std
        with open(self.fo.name, 'w') as f:
            w = csv.writer(f, dialect="excel", delimiter=";")
            w.writerow(rowdata)
        with open(self.fo.name) as f:
            std_ret = f.read().strip()

        # validation
        self.assertEqual(ret, std_ret)

    def test_writer_opt_delimiter_verticalbar(self):
        rowdata = [1, 2, -4.1, "Hello", True, False, None]
        # fcsv
        w = fcsv.writer(self.fo.name, delimiter="|")
        w.writerow(rowdata)
        with open(self.fo.name) as f:
            ret = f.read().strip()
            self.assertEqual("1|2|-4.1|Hello|True|False|", ret)

        # std
        with open(self.fo.name, 'w') as f:
            w = csv.writer(f, delimiter="|")
            w.writerow(rowdata)

        # validation
        with open(self.fo.name) as f:
            std_ret = f.read().strip()
            self.assertEqual(ret, std_ret)


class TestReaderSmoke(unittest.TestCase):

    def setUp(self):
        self.fo = NamedTemporaryFile("w", delete=False)

    def tearDown(self):
        os.remove(self.fo.name)

    def test_reader(self):
        self.fo.write("1,2,-4.1,Hello\n")
        self.fo.close()
        ret = [r for r in fcsv.reader(self.fo.name)]
        self.assertEqual(ret, [["1", "2", "-4.1", "Hello"]])
        with open(self.fo.name) as f:
            std_ret = [r for r in csv.reader(f)]
            self.assertEqual(ret, std_ret)

    def test_reader_opt_delimiter(self):
        self.fo.write("1;2;-4.1;Hello\n")
        self.fo.close()
        ret = [r for r in fcsv.reader(self.fo.name, delimiter=";")]
        self.assertEqual(ret, [["1", "2", "-4.1", "Hello"]])
        with open(self.fo.name) as f:
            std_ret = [r for r in csv.reader(f, delimiter=";")]
            self.assertEqual(ret, std_ret)

    def test_reader_opt_doublequote(self):
        self.fo.write('"1","2","-4.1","Hello"\n')
        self.fo.close()
        ret = [r for r in fcsv.reader(self.fo.name)]
        self.assertEqual(ret, [["1", "2", "-4.1", "Hello"]])
        with open(self.fo.name) as f:
            std_ret = [r for r in csv.reader(f)]
            self.assertEqual(ret, std_ret)

    def test_reader_opt_singlequote(self):
        self.fo.write("'1','2','-4.1','Hello'\n")
        self.fo.close()
        ret = [r for r in fcsv.reader(self.fo.name)]
        self.assertEqual(ret, [["'1'", "'2'", "'-4.1'", "'Hello'"]])
        with open(self.fo.name) as f:
            std_ret = [r for r in csv.reader(f)]
            self.assertEqual(ret, std_ret)


if __name__ == '__main__':
    unittest.main()
