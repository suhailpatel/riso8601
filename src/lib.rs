extern crate pyo3;

use pyo3::create_exception;
use pyo3::exceptions::Exception;
use pyo3::prelude::*;
use pyo3::types::PyDateTime;
use pyo3::wrap_pyfunction;

create_exception!(riso8601, ParseError, Exception);

#[pyfunction]
fn parse_datetime<'p>(py: Python<'p>, input: &str) -> PyResult<&'p PyDateTime> {
    // Expected formats
    // - 2020-07-16T19:20
    // - 2020-07-16T19:20:01
    // - 2020-07-16T19:20:01.000001
    // - 2020-07-16T19:20:01+01:00
    // - 2020-07-16T19:20:01.000001+01:00

    // Sanity check for a minimum length of 16 characters
    if input.len() < 16 {
        return Err(ParseError::py_err("invalid time string"));
    }

    let year: i32 = match input[0..=3].parse() {
        Ok(val) if val > 0 => val,
        Ok(_) => return Err(ParseError::py_err("year needs to be above 0")),
        _ => return Err(ParseError::py_err("invalid time string")),
    };

    let month: u8 = match input[5..=6].parse() {
        Ok(val) if val >= 1 && val <= 12 => val,
        Ok(_) => return Err(ParseError::py_err("month needs to be between 1-12")),
        _ => return Err(ParseError::py_err("invalid time string")),
    };

    let day: u8 = match input[8..=9].parse() {
        Ok(val) if val >= 1 && val <= 31 => val,
        Ok(_) => return Err(ParseError::py_err("day needs to be between 1 and 31")),
        _ => return Err(ParseError::py_err("invalid time string")),
    };

    let hour: u8 = match input[11..=12].parse() {
        Ok(val) if val <= 23 => val,
        Ok(_) => return Err(ParseError::py_err("hour needs to be between 00 and 23")),
        _ => return Err(ParseError::py_err("invalid time string")),
    };

    let minute: u8 = match input[14..=15].parse() {
        Ok(val) if val <= 23 => val,
        Ok(_) => return Err(ParseError::py_err("minute needs to be between 00 and 59")),
        _ => return Err(ParseError::py_err("invalid time string")),
    };

    // At this point, if we are done with our input, it's time to return!
    if input.len() == 16 {
        return PyDateTime::new(py, year, month, day, hour, minute, 0, 0, None);
    }

    // Check if the next character is a colon or a timezone identifier
    let seconds: u8 = match input[16..17].as_ref() {
        "T" => 0,
        ":" if input.len() >= 19 => match input[17..=18].parse() {
            Ok(val) if val <= 59 => val,
            Ok(_) => return Err(ParseError::py_err("seconds needs to be between 00 and 59")),
            _ => return Err(ParseError::py_err("invalid time string")),
        },
        _ => return Err(ParseError::py_err("invalid time string")),
    };

    // Define our UTC timezone at this point
    let datetime = py.import("datetime").map_err(|e| e.print(py)).unwrap();
    let timezone = datetime.get("timezone").unwrap();
    let utc = timezone.getattr("utc").unwrap().to_object(py);

    PyDateTime::new(py, year, month, day, hour, minute, seconds, 1, Some(&utc))
}

#[pymodule]
fn riso8601(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(parse_datetime))?;

    Ok(())
}
