# riso8601

riso8601 is a Python module which parses ISO8601 datetimes. It is written in Rust 🦀

🚨 This module has not been extensively tested against the entire corpus of ISO8601 datetimes. It's a toy implementation built against Python 3.7. If you need something more battle-tested, check out the [ciso8601](https://pypi.org/project/ciso8601/) library instead. 

# Usage

```
>>> import riso8601
>>> riso8601.parse_datetime("2020-07-16T19:20:10.10000-12:00")
datetime.datetime(2020, 7, 16, 19, 20, 10, 10000, tzinfo=datetime.timezone(datetime.timedelta(seconds=-43200)))
```

# Author

Suhail Patel <me@suhailpatel.com>
