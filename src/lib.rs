#![feature(proc_macro, specialization)]

extern crate csv;
extern crate pyo3;

use std::fs;
use std::io::{BufReader, BufWriter};
use csv::{QuoteStyle, ReaderBuilder, WriterBuilder, Terminator};
use pyo3::prelude::*;


fn pyobj2str(obj: &PyObjectRef) -> Result<String, String> {
    match obj.extract::<String>() {
        Ok(v) => return Ok(v),
        Err(_) => {},
    }
    match obj.extract::<&PyBytes>() {
        Ok(v) => {
            let s = String::from_utf8(v.data().to_vec());
            match s {
                Err(e) => return Err(format!("undecoded data: {:?}", e)),
                _ => {},
            }
            let s = s.unwrap();
            return Ok(s);
        },
        Err(_) => {},
    }
    match obj.extract::<&PyBool>() {
        Ok(v) => {
            if v.is_true() {
                return Ok("True".to_string());
            } else {
                return Ok("False".to_string());
            }
        }
        Err(_) => {},
    }
    match obj.extract::<f64>() {
        Ok(v) => {
            return Ok(format!("{}", v));
        },
        Err(_) => {},
    }
    match obj.extract::<i64>() {
        Ok(v) => {
            return Ok(format!("{}", v));
        },
        Err(_) => {},
    }
    if obj.is_none() {
        return Ok("".to_string());
    }

    Err("invalid field type".to_string())
}

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
    #[args(path, kwargs="**")]
    fn __new__(obj: &PyRawObject, path: String, kwargs: Option<&PyDict>) -> PyResult<()> {
        let delimiter = if kwargs.is_some() {
            let kwargs = kwargs.expect("hoge");
            match kwargs.get_item("delimiter") {
                Some(x) => x.extract::<String>().expect("fuga").as_bytes()[0],
                None => b',',
            }
        } else {
            b','
        };
        let fp = BufWriter::new(fs::File::create(path.as_str()).expect("fail create file"));
        let wtr = WriterBuilder::new()
            .flexible(true)
            .terminator(Terminator::CRLF)
            .quote_style(QuoteStyle::Necessary)
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
            let x = x.expect("invalid data");
            match pyobj2str(x) {
                Ok(s) => {
                    let _ = self._wtr.write_field(s.as_bytes());
                },
                Err(_) => panic!("invalid type"), /* TODO: handle error */
            }
        }
        let _ = self._wtr
            .write_record(None::<&[u8]>)
            .expect("fail write none record");
        self._wtr.flush()?;
        Ok(())
    }

    fn writerows(&mut self, py: Python, args: PyObject) -> PyResult<()> {
        let itero = PyIterator::from_object(py, &args).expect("fail get iter");
        for arg in itero {
            let v = PyIterator::from_object(py, arg.unwrap()).expect("fail get iter");
            for item in v {
                let sitem = item.unwrap();
                match pyobj2str(sitem) {
                    Ok(s) => {
                        let _ = self._wtr.write_field(s.as_bytes());
                    },
                    Err(_) => panic!("invalid type"), /* TODO: handle error */
                }
            }
            let _ = self._wtr
                .write_record(None::<&[u8]>)
                .expect("fail write none record");
        }
        self._wtr.flush()?;
        Ok(())
    }
}

#[py::methods]
impl Reader {
    #[new]
    #[args(path, dialect, kwargs="**")]
    fn __new__(obj: &PyRawObject, path: String, _args: Option<PyObject>, kwargs: Option<&PyDict>) -> PyResult<()> {
        let delimiter = if kwargs.is_some() {
            let kwargs = kwargs.unwrap();
            match kwargs.get_item("delimiter") {
                Some(x) => x.extract::<String>().unwrap().as_bytes()[0],
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
            let xx = x.unwrap();
            result.push(xx.iter().collect::<Vec<&str>>().to_object(py));
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
                Ok(Some(record.iter().collect::<Vec<&str>>().to_object(py)))
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
