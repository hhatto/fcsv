#![feature(proc_macro, specialization)]

extern crate csv;
extern crate pyo3;

use std::fs;
use std::io::BufWriter;
use csv::{QuoteStyle, WriterBuilder};
use pyo3::prelude::*;

#[py::class]
struct Writer {
    _wtr: csv::Writer<BufWriter<fs::File>>,
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

#[py::modinit(fcsv)]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Writer>()?;

    Ok(())
}
