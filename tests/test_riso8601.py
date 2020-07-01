import datetime
from datetime import timedelta, timezone

import pytest
import riso8601


def test_naive_times():
    expected = datetime.datetime(2014, 1, 9, 21, 48)

    assert expected == riso8601.parse_datetime("20140109T2148")
    assert expected == riso8601.parse_datetime("2014-01-09T2148")
    assert expected == riso8601.parse_datetime("20140109T21:48")
    assert expected == riso8601.parse_datetime("2014-01-09T21:48")


def test_naive_times_with_seconds():
    expected = datetime.datetime(2014, 1, 9, 21, 48, 53)

    assert expected == riso8601.parse_datetime("20140109T214853")
    assert expected == riso8601.parse_datetime("2014-01-09T214853")
    assert expected == riso8601.parse_datetime("20140109T21:48:53")
    assert expected == riso8601.parse_datetime("2014-01-09T21:48:53")
    assert expected == riso8601.parse_datetime("2014-01-09T21:4853")  # this one might be legally invalid?
    assert expected == riso8601.parse_datetime("2014-01-09T2148:53")  # this one might be legally invalid?


def test_naive_times_with_microseconds():
    expected = datetime.datetime(2014, 1, 9, 21, 48, 53, 0)

    assert expected == riso8601.parse_datetime("20140109T214853.000000")
    assert expected == riso8601.parse_datetime("2014-01-09T214853.0")
    assert expected == riso8601.parse_datetime("20140109T21:48:53.0")
    assert expected == riso8601.parse_datetime("2014-01-09T21:48:53.000")
    assert expected == riso8601.parse_datetime("2014-01-09T21:4853.0000")  # this one might be legally invalid?
    assert expected == riso8601.parse_datetime("2014-01-09T2148:53.00000")  # this one might be legally invalid?


def test_edge_case_times():
    assert datetime.datetime(2020, 1, 1, 0, 0) == riso8601.parse_datetime("2020-01-01T00:00")
    assert datetime.datetime(2020, 1, 1, 0, 0, 0) == riso8601.parse_datetime("2020-01-01T00:00:00")
    assert datetime.datetime(2020, 1, 1, 0, 0, 0, 0) == riso8601.parse_datetime("2020-01-01T00:00:00.000000")

    assert datetime.datetime(2020, 12, 31, 23, 59) == riso8601.parse_datetime("2020-12-31T23:59")
    assert datetime.datetime(2020, 12, 31, 23, 59, 59) == riso8601.parse_datetime("2020-12-31T23:59:59")
    assert datetime.datetime(2020, 12, 31, 23, 59, 59, 999999) == riso8601.parse_datetime("2020-12-31T23:59:59.999999")


def test_tz_times():
    assert datetime.datetime(2014, 1, 9, 21, 48, tzinfo=timezone.utc) == riso8601.parse_datetime("2014-01-09T21:48Z")

    tz = timezone(timedelta(seconds=43200))
    assert datetime.datetime(2014, 1, 9, 21, 48, tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48+12")
    assert datetime.datetime(2014, 1, 9, 21, 48, 30, tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48:30+12")
    assert datetime.datetime(2014, 1, 9, 21, 48, 30, 99999,
                             tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48:30.99999+12")
    assert datetime.datetime(2014, 1, 9, 21, 48, 30, 99999,
                             tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48:30.99999+12:00")

    tz = timezone(timedelta(seconds=-43200))
    assert datetime.datetime(2014, 1, 9, 21, 48, tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48-12")
    assert datetime.datetime(2014, 1, 9, 21, 48, 30, tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48:30-12")
    assert datetime.datetime(2014, 1, 9, 21, 48, 30, 99999,
                             tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48:30.99999-12")
    assert datetime.datetime(2014, 1, 9, 21, 48, 30, 99999,
                             tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48:30.99999-12:00")

    tz = timezone(timedelta(seconds=1800))
    assert datetime.datetime(2014, 1, 9, 21, 48, tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48+00:30")
    assert datetime.datetime(2014, 1, 9, 21, 48, 30, 99999,
                             tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48:30.99999+00:30")

    tz = timezone(timedelta(seconds=-1800))
    assert datetime.datetime(2014, 1, 9, 21, 48, tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48-00:30")
    assert datetime.datetime(2014, 1, 9, 21, 48, 30, 99999,
                             tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48:30.99999-00:30")

    tz = timezone(timedelta(seconds=0))
    assert datetime.datetime(2014, 1, 9, 21, 48, tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48-00")
    assert datetime.datetime(2014, 1, 9, 21, 48, 30, 99999,
                             tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48:30.99999+00")
    assert datetime.datetime(2014, 1, 9, 21, 48, tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48-00:00")
    assert datetime.datetime(2014, 1, 9, 21, 48, 30, 99999,
                             tzinfo=tz) == riso8601.parse_datetime("2014-01-09T21:48:30.99999+00:00")


def test_invalid_times():
    invalids = [
        "-5000-01-01T00:00:00",   # bad year
        "2014-00-01T00:00:00",    # bad month
        "2014-13-01T00:00:00",    # bad month
        "2014-10-00T00:00:00",    # bad day
        "2014-10-32T00:00:00",    # bad day
        "2014-10-32T24:00:00",    # bad hour
        "2014-10-32T00:60:00",    # bad minute
        "2014-10-32T00:00:60",    # bad second
        "2014-10-32T00:00",       # no second
        "2014-10-32T00:00:00.",   # no microsecond
    ]

    for dt_str in invalids:
        with pytest.raises(Exception):
            riso8601.parse_datetime(dt_str)
