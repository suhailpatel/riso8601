extern crate pyo3;

use pyo3::create_exception;
use pyo3::exceptions::Exception;
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDelta};
use pyo3::wrap_pyfunction;

create_exception!(riso8601, ParseError, Exception);

#[pyfunction]
fn parse_datetime<'p>(py: Python<'p>, input: &str) -> PyResult<&'p PyDateTime> {
    // Expected formats
    // - 20200716T1920
    // - 2020-07-16T19:20
    // - 2020-07-16T19:20:01
    // - 2020-07-16T19:20:01.000001
    // - 2020-07-16T19:20:01+01:00
    // - 2020-07-16T19:20:01.000001+01:00

    // Sanity check for a minimum length of 13 characters (shortest)
    if input.len() < 13 {
        return Err(ParseError::py_err("invalid time string"));
    }

    // We need to keep track of our index which we are currently pointing to
    let mut point = 0;

    let year: i32 = match input[point..point + 4].parse() {
        Ok(val) if val > 0 => val,
        Ok(_) => return Err(ParseError::py_err("year needs to be above 0")),
        _ => return Err(ParseError::py_err("invalid time string (year)")),
    };

    // Do a dash check, advance our point position accordingly
    match input[point + 4..point + 5].as_ref() {
        "-" => point = point + 5,
        _ => point = point + 4,
    }

    let month: u8 = match input[point..point + 2].parse() {
        Ok(val) if val >= 1 && val <= 12 => val,
        Ok(_) => return Err(ParseError::py_err("month needs to be between 1-12")),
        _ => return Err(ParseError::py_err("invalid time string (month)")),
    };

    // Do a dash check, advance our point position accordingly
    match input[point + 2..point + 3].as_ref() {
        "-" => point = point + 3,
        _ => point = point + 2,
    };

    let day: u8 = match input[point..point + 2].parse() {
        Ok(val) if val >= 1 && val <= 31 => val,
        Ok(_) => return Err(ParseError::py_err("day needs to be between 1 and 31")),
        _ => return Err(ParseError::py_err("invalid time string (day)")),
    };

    // Do a check for a 'T', advance our point position accordingly
    match input[point + 2..point + 3].as_ref() {
        "T" => point = point + 3,
        _ => point = point + 2,
    };

    let hour: u8 = match input[point..point + 2].parse() {
        Ok(val) if val <= 23 => val,
        Ok(_) => return Err(ParseError::py_err("hour needs to be between 00 and 23")),
        _ => return Err(ParseError::py_err("invalid time string (hour)")),
    };

    // Do a colon check, advance our point position accordingly
    match input[point + 2..point + 3].as_ref() {
        ":" => point = point + 3,
        _ => point = point + 2,
    };

    let minute: u8 = match input[point..point + 2].parse() {
        Ok(val) if val <= 23 => val,
        Ok(_) => return Err(ParseError::py_err("minute needs to be between 00 and 59")),
        _ => return Err(ParseError::py_err("invalid time string (minute)")),
    };

    // Advance our pointer by 2 characters as we have completed minute
    point = point + 2;

    // At this point, if we are done with our input, it's time to return!
    if input.len() == point {
        return PyDateTime::new(py, year, month, day, hour, minute, 0, 0, None);
    }

    // Check if the next character is a colon (for seconds) or a timezone identifier (+/-)
    let second: u8 = match input[point..point + 1].as_ref() {
        "+" | "-" | "Z" => 0,
        ":" if input.len() >= 19 => {
            point = point + 3; // ':' plus two digits
            match input[point - 2..point].parse() {
                Ok(val) if val <= 59 => val,
                Ok(_) => return Err(ParseError::py_err("seconds needs to be between 00 and 59")),
                _ => return Err(ParseError::py_err("invalid time string (second)")),
            }
        }
        _ => return Err(ParseError::py_err("invalid time string (second)")),
    };

    // Check if we've reached the end of our input yet?
    if input.len() == point {
        return PyDateTime::new(py, year, month, day, hour, minute, second, 0, None);
    }

    // Check if the next character is a microsecond dot or a timezone identifier (+/-/)
    let ms: u32 = match input[point..point + 1].as_ref() {
        "+" | "-" | "Z" => 0,
        "." if input.len() > point => {
            // At this point, we are reading microseconds but the precision here
            // is entirely arbitrary, so collect as many digits as we can from
            // index + 1 onwards
            let ms_str = input[point + 1..]
                .chars()
                .take_while(|c| c.is_digit(10))
                .collect::<String>();

            point = point + 1 + ms_str.len(); // '.' plus the number of collected digits
            match ms_str.parse() {
                Ok(val) if val <= 999999 => val,
                Ok(_) => {
                    return Err(ParseError::py_err(
                        "microseconds needs to be between 0 and 999999",
                    ))
                }
                _ => return Err(ParseError::py_err("invalid time string (microsecond)")),
            }
        }
        _ => return Err(ParseError::py_err("invalid time string (microsecond)")),
    };

    // Check if we've reached the end of our input yet?
    if input.len() == point {
        return PyDateTime::new(py, year, month, day, hour, minute, second, ms, None);
    }

    let tz: PyObject = match parse_timezone(py, input[point..].as_ref()) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    return PyDateTime::new(py, year, month, day, hour, minute, second, ms, Some(&tz));
}

fn parse_timezone<'p>(py: Python<'p>, input: &str) -> PyResult<PyObject> {
    // Import the stuff we're going to need from the stdlib
    let pydatetime = py.import("datetime").map_err(|e| e.print(py)).unwrap();
    let pytimezone = pydatetime.get("timezone").unwrap();

    match input[0..1].as_ref() {
        "Z" if input.len() == 1 => {
            return pytimezone.getattr("utc").unwrap().extract();
        }
        "+" | "-" if input.len() - input.len() >= 3 || input.len() <= 6 => {
            let mut point = 0;

            // Figure out if we got a positive or negative
            let multiplier = match input[point..point + 1].as_ref() {
                "-" => -1,
                _ => 1,
            };

            let hr_val: i32 = match input[point + 1..point + 3].parse() {
                Ok(val) => val,
                _ => return Err(ParseError::py_err("invalid time string (tz hr)")),
            };

            // Short circuit if we got nothing else
            if input.len() == 3 {
                let tzsecs = hr_val * 60 * 60;
                let delta = PyDelta::new(py, 0, multiplier * tzsecs, 0, false).unwrap();
                return pydatetime.call1("timezone", (delta,)).unwrap().extract();
            }

            point = point + 3;

            // Check for a colon, move past it if present
            match input[point..point + 1].as_ref() {
                ":" => point = point + 1,
                _ => (),
            };

            // Make sure we have two more items remaining for our min specifier
            if input.len() - point != 2 {
                return Err(ParseError::py_err("invalid time string (timezone)"));
            }

            let min_val: i32 = match input[point..point + 2].parse() {
                Ok(val) => val,
                _ => return Err(ParseError::py_err("invalid time string (tz min)")),
            };

            let tzsecs = (hr_val * 60 * 60) + (min_val * 60);
            let delta = PyDelta::new(py, 0, multiplier * tzsecs, 0, false).unwrap();
            return pydatetime.call1("timezone", (delta,)).unwrap().extract();
        }
        _ => return Err(ParseError::py_err("invalid time string (timezone)")),
    }
}

#[pymodule]
fn riso8601(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(parse_datetime))?;

    Ok(())
}
