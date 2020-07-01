# riso8601

riso8601 is a Python module which parses ISO8601 datetimes. It is written in Rust 🦀

🚨 This module has not been extensively tested against the entire corpus of ISO8601 datetimes. It's a toy implementation built against Python 3.7. If you need something more battle-tested, check out the [ciso8601](https://pypi.org/project/ciso8601/) library instead. 

## Supported Subset

This module does not support the full subset of ISO8601 times. It supports:
- `YYYYMMDDThhmmss`
- `YYYY-MM-DDThh:mm:ss`
- `YYYY-MM-DDThh:mm:ss.ssssss`

Timezones are also supported as a suffix in the following formats:
- `Z`
- `+hh` and `-hh`
- `+hhmm` and `-hhmm`
- `+hh:mm` and `-hh:mm`

Some derivations may also be supported but that behaviour is subject to change. If you are unsure, check out the `tests`.

# Usage

This is a toy project so it's not on PyPI. You can check out the source and install it locally (tested with Rust 1.40.0 and Python 3.7):
```
$ pip install -r requirements-dev.txt
$ python setup.py install
```

To use it once installed:
```
>>> import riso8601
>>> riso8601.parse_datetime("2020-07-16T19:20:10.10000-12:00")
datetime.datetime(2020, 7, 16, 19, 20, 10, 10000, tzinfo=datetime.timezone(datetime.timedelta(seconds=-43200)))
```

# Author

Suhail Patel <me@suhailpatel.com>
