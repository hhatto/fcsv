#![feature(proc_macro, specialization)]

extern crate csv;
extern crate pyo3;

use std::fs;
use std::io::{BufReader, BufWriter};
use csv::{QuoteStyle, ReaderBuilder, WriterBuilder, Terminator};
use pyo3::prelude::*;

#[py::class]
struct Writer {
    _wtr: csv::Writer<BufWriter<fs::File>>,
    token: PyToken,
}

#[py::class]
struct Reader {
    _rdr: csv::Reader<BufReader<fs::File>>,
    token: PyToken,
}

#[py::methods]
impl Writer {
    #[new]
    #[args(path, args = "*", kwargs = "**")]
    fn __new__(obj: &PyRawObject, path: String, _args: Option<&PyTuple>, kwargs: Option<&PyDict>) -> PyResult<()> {
        let delimiter = if kwargs.is_some() {
            // TODO: use macro
            let kwargs = kwargs.unwrap();
            match kwargs.get_item("delimiter") {
                Some(x) => {
                    String::from(
                        PyString::try_from(x)
                            .expect("fail from_object")
                            .to_string_lossy(),
                    ).as_bytes()[0]
                }
                None => b',',
            }
        } else {
            b','
        };
        let fp = BufWriter::new(fs::File::create(path.as_str()).expect("fail create file"));
        let wtr = WriterBuilder::new()
            .flexible(true)
            .terminator(Terminator::CRLF)
            .quote_style(QuoteStyle::NonNumeric)
            .delimiter(delimiter)
            .from_writer(fp);
        obj.init(|t| Writer {
            _wtr: wtr,
            token: t,
        })
    }

    fn writerow(&mut self, py: Python, arg: PyObject) -> PyResult<()> {
        let itero = PyIterator::from_object(py, &arg).expect("fail get iter");
        for x in itero {
            let _ = self._wtr.write_field(String::from(
                    PyString::try_from(x.unwrap())
                    .expect("fail from_object")
                    .to_string_lossy(),
                    ));
        }
        let _ = self._wtr
            .write_record(None::<&[u8]>)
            .expect("fail write none record");
        Ok(())
    }

    fn writerows(&mut self, py: Python, args: PyObject) -> PyResult<()> {
        let itero = PyIterator::from_object(py, &args).expect("fail get iter");
        for arg in itero {
            let v = PyIterator::from_object(py, arg.unwrap()).expect("fail get iter");
            for item in v {
                let sitem = item.unwrap();
                let _ = self._wtr.write_field(String::from(
                    PyString::try_from(sitem)
                        .expect("fail from_object")
                        .to_string_lossy(),
                ));
            }
            let _ = self._wtr
                .write_record(None::<&[u8]>)
                .expect("fail write none record");
        }
        Ok(())
    }
}

#[py::methods]
impl Reader {
    #[new]
    #[args(path, args = "*", kwargs = "**")]
    fn __new__(obj: &PyRawObject, path: String, _args: Option<&PyTuple>, kwargs: Option<&PyDict>) -> PyResult<()> {
        let delimiter = if kwargs.is_some() {
            // TODO: use macro
            let kwargs = kwargs.unwrap();
            match kwargs.get_item("delimiter") {
                Some(x) => {
                    String::from(
                        PyString::try_from(x)
                            .expect("fail from_object")
                            .to_string_lossy(),
                    ).as_bytes()[0]
                }
                None => b',',
            }
        } else {
            b','
        };
        let fp = BufReader::new(fs::File::open(path.as_str()).expect("fail create file"));
        let rdr = ReaderBuilder::new()
            .flexible(true)
            .terminator(Terminator::CRLF)
            .has_headers(false)
            .delimiter(delimiter)
            .from_reader(fp);
        obj.init(|t| Reader {
            _rdr: rdr,
            token: t,
        })
    }

    fn read(&mut self, py: Python) -> PyResult<PyObject> {
        let cnt = self._rdr.records().count();
        let mut result = Vec::with_capacity(cnt);
        let pos = csv::Position::new();
        let _ = self._rdr.seek(pos);
        for x in self._rdr.records() {
            let mut v = vec![];
            let xx = x.unwrap();
            for xxx in xx.iter() {
                v.push(PyString::new(py, xxx));
            }
            result.push(v.to_object(py));
        }
        let obj = result.to_object(py);
        Ok(obj)
    }
}

#[py::proto]
impl PyIterProtocol for Reader {
    fn __iter__(&mut self) -> PyResult<PyObject> {
        Ok(self.into())
    }

    fn __next__(&mut self) -> PyResult<Option<PyObject>> {
        let mut record = csv::StringRecord::new();
        match self._rdr.read_record(&mut record) {
            Ok(true) => {
                let py = self.py();
                let v: Vec<Py<PyString>> = record.iter().map(|s| PyString::new(py, s)).collect();
                Ok(Some(v.to_object(py)))
            }
            _ => Err(exc::StopIteration::new("stop")),
        }
    }
}

#[py::modinit(_fcsv)]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Writer>()?;
    m.add_class::<Reader>()?;

    Ok(())
}
