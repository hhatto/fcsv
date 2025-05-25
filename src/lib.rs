use std::fs;
use std::io::{BufReader, BufWriter};
use csv::{QuoteStyle, ReaderBuilder, WriterBuilder, Terminator};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyBool, PyDict, PyIterator};
use pyo3::exceptions;


fn pyobj2str(obj: &Bound<'_, PyAny>) -> Result<String, String> {
    match obj.extract::<String>() {
        Ok(v) => return Ok(v),
        Err(_) => {},
    }
    match obj.downcast::<PyBytes>() {
        Ok(v) => {
            let s = String::from_utf8(v.as_bytes().to_vec());
            match s {
                Err(e) => return Err(format!("undecoded data: {:?}", e)),
                _ => {},
            }
            let s = s.unwrap();
            return Ok(s);
        },
        Err(_) => {},
    }
    match obj.downcast::<PyBool>() {
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

#[pyclass]
struct Writer {
    _wtr: csv::Writer<BufWriter<fs::File>>,
}

#[pyclass]
struct Reader {
    _rdr: csv::Reader<BufReader<fs::File>>,
}

#[pymethods]
impl Writer {
    #[new]
    fn __new__(path: String, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let delimiter = if kwargs.is_some() {
            let kwargs = kwargs.expect("kwargs parse error");
            match kwargs.get_item("delimiter")? {
                Some(x) => x.extract::<String>().expect("fail to new writer object").as_bytes()[0],
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
        Ok(Writer { _wtr: wtr })
    }

    fn writerow(&mut self, arg: &Bound<'_, PyAny>) -> PyResult<()> {
        let itero = PyIterator::from_object(arg).expect("fail get iter");
        for x in itero {
            let x = x.expect("invalid data");
            match pyobj2str(&x) {
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

    fn writerows(&mut self, args: &Bound<'_, PyAny>) -> PyResult<()> {
        let itero = PyIterator::from_object(args).expect("fail get iter");
        for arg in itero {
            let v = PyIterator::from_object(&arg.unwrap()).expect("fail get iter");
            for item in v {
                let sitem = item.unwrap();
                match pyobj2str(&sitem) {
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

#[pymethods]
impl Reader {
    #[new]
    fn __new__(path: String, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let delimiter = if kwargs.is_some() {
            let kwargs = kwargs.unwrap();
            match kwargs.get_item("delimiter")? {
                Some(x) => x.extract::<String>().unwrap().as_bytes()[0],
                None => b',',
            }
        } else {
            b','
        };
        let f = fs::File::open(path.as_str());
        match f {
            Err(e) => return Err(exceptions::PyIOError::new_err(format!("{:?}", e))),
            _ => {}
        }
        let f = f.unwrap();
        let fp = BufReader::new(f);
        let rdr = ReaderBuilder::new()
            .flexible(true)
            .terminator(Terminator::CRLF)
            .has_headers(false)
            .delimiter(delimiter)
            .from_reader(fp);
        Ok(Reader { _rdr: rdr })
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        let mut record = csv::StringRecord::new();
        match self._rdr.read_record(&mut record) {
            Ok(true) => {
                Ok(Some(record.iter().collect::<Vec<&str>>().into_pyobject(py)?.into_any().unbind()))
            }
            _ => Err(exceptions::PyStopIteration::new_err("stop")),
        }
    }

    fn read(&mut self, py: Python) -> PyResult<PyObject> {
        let cnt = self._rdr.records().count();
        let mut result = Vec::with_capacity(cnt);
        let pos = csv::Position::new();
        let _ = self._rdr.seek(pos);
        for x in self._rdr.records() {
            let xx = x.unwrap();
            result.push(xx.iter().collect::<Vec<&str>>().into_pyobject(py)?.into_any().unbind());
        }
        let obj = result.into_pyobject(py)?.into_any().unbind();
        Ok(obj)
    }
}

#[pymodule]
#[pyo3(name = "_fcsv")]
fn init_mod(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Writer>()?;
    m.add_class::<Reader>()?;

    Ok(())
}
