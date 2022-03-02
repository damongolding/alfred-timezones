import pendulum
from pendulum import DateTime
import sys
import json
import re
from re import Match

timezones = [
    ("Europe/London", "GMT"),
    # ("Etc/GMT", "UTC"),
    ("America/Los_Angeles", "PST"),
    ("America/New_York", "EST"),
    ("America/Chicago", "CST"),
    ("America/Denver", "MST"),
]

PATTERN_12 = r"(0?[0-9]|1[0-9]|2[0-3]):([0-5][0-9])+(am|pm)+\s([a-zA-Z]+)"
PATTERN_24 = r"(0?[0-9]|1[0-9]|2[0-3]):([0-5][0-9])+\s([a-zA-Z]+)"


def is_12_hour_format(input: str) -> Match[str] | None:
    pattern = re.compile(PATTERN_12)
    return re.match(pattern, input)


def is_24_hour_format(input: str) -> Match[str] | None:
    pattern = re.compile(PATTERN_24)
    return re.match(pattern, input)


def get_regex_groups(input: str, pattern: str) -> tuple[str]:
    pattern = re.compile(pattern)
    matches = re.match(pattern, input)
    groups = matches.groups()
    return groups


def display_24_hour_times(groups: tuple) -> dict:
    hour = int(groups[0])
    minute = int(groups[1])
    submitted_timezone = groups[2]

    year = pendulum.now().year
    month = pendulum.now().month
    day = pendulum.now().day

    timezone: tuple = list(filter(lambda x: x[1].lower() ==
                           submitted_timezone.lower(), timezones))[0]

    time = pendulum.datetime(year=year, month=month,
                             day=day, hour=hour, minute=minute, tz=timezone[0])

    return get_times(time=time, time_format="24", submitted_timezone=submitted_timezone)


def display_12_hour_times(groups: tuple) -> dict:
    hour = int(groups[0])
    minute = int(groups[1])
    am_pm = groups[2]
    submitted_timezone = groups[3]

    year = pendulum.now().year
    month = pendulum.now().month
    day = pendulum.now().day

    if am_pm == 'am' and hour == 12:
        hour = 00
    elif am_pm == 'pm' and hour != 12:
        hour = hour + 12

    #  filter array of tuples to only include timezones that match the submitted timezone
    timezone: tuple = list(filter(lambda x: x[1].lower() ==
                           submitted_timezone.lower(), timezones))[0]

    time = pendulum.datetime(year=year, month=month,
                             day=day, hour=hour, minute=minute, tz=timezone[0])

    return get_times(time=time, time_format="12", submitted_timezone=submitted_timezone)


def get_times(time: DateTime, time_format: str, submitted_timezone: str) -> dict:

    time_format = "h:mmA" if time_format == "12" else "HH:mm"
    times_to_return = {'items': []}

    for timezone in timezones:
        if timezone[1].lower() == submitted_timezone.lower():
            continue

        time_to_return = f"{time.in_timezone(timezone[0]).format(time_format, locale='en')} {timezone[1]}"

        times_to_return['items'].append(
            {"title": time_to_return, "arg": time_to_return, 'icon': {'path': '~/GIT/pendulum/images/clock.png'}})

    return times_to_return


def main():

    if len(sys.argv) != 2:
        sys.exit(0)

    time_argument = sys.argv[1]

    if is_12_hour_format(time_argument):
        groups = get_regex_groups(time_argument, PATTERN_12)
        times = display_12_hour_times(groups)
        print(json.dumps(times, indent=4, ensure_ascii=False))

    elif is_24_hour_format(time_argument):
        groups = get_regex_groups(time_argument, PATTERN_24)
        times = display_24_hour_times(groups)
        print(json.dumps(times, indent=4, ensure_ascii=False))

    else:
        print(json.dumps(
            {'items': [{'title': "Incorrect format", "suntitle": "e.g. 10:34am gmt"}]}, indent=4))


if __name__ == "__main__":
    main()
