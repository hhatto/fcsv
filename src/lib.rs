#![feature(proc_macro, specialization)]

extern crate csv;
extern crate pyo3;

use std::fs;
use std::io::{BufReader, BufWriter};
use csv::{QuoteStyle, WriterBuilder, ReaderBuilder};
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
    fn __new__(obj: &PyRawObject, path: String) -> PyResult<()> {
        let fp = BufWriter::new(fs::File::create(path.as_str()).expect("fail create file"));
        let wtr = WriterBuilder::new()
            .quote_style(QuoteStyle::NonNumeric)
            .from_writer(fp);
        obj.init(|t| Writer {
            _wtr: wtr,
            token: t,
        })
    }

    fn writerow(&mut self, _py: Python, arg: &PyTuple) -> PyResult<()> {
        for x in arg.iter() {
            let _ = self._wtr.write_field(String::from(
                PyString::try_from(x.as_ref())
                    .expect("fail from_object")
                    .to_string_lossy(),
            ));
        }
        let _ = self._wtr
            .write_record(None::<&[u8]>)
            .expect("fail write none record");
        Ok(())
    }

    fn writerows(&mut self, _py: Python, args: &PyTuple) -> PyResult<()> {
        for arg in args.iter() {
            let v = PyIterator::from_object(_py, arg).expect("fail get iter");
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
    fn __new__(obj: &PyRawObject, path: String) -> PyResult<()> {
        let fp = BufReader::new(fs::File::open(path.as_str()).expect("fail create file"));
        let rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(fp);
        obj.init(|t| Reader {
            _rdr: rdr,
            token: t,
        })
    }

    fn read(&mut self, py: Python) -> PyResult<PyObject> {
        let mut result = vec![];
        for x in self._rdr.records() {
            let mut v = vec![];
            for xx in x.iter() {
                for xxx in xx.iter() {
                v.push(PyString::new(py, xxx));
                }
            }
            result.push(v.to_object(py));
        }
        let obj = result.to_object(py);
        Ok(obj)
    }
}

#[py::modinit(fcsv)]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Writer>()?;
    m.add_class::<Reader>()?;

    Ok(())
}
