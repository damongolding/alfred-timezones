import pendulum
from pendulum import DateTime
import sys
import json
import re
from re import Match
import os


class TimeZones:

    PATTERN_12 = r"(0?[0-9]|1[0-9]|2[0-3]):([0-5][0-9])+(am|pm)+\s([a-zA-Z]+)"
    PATTERN_24 = r"(0?[0-9]|1[0-9]|2[0-3]):([0-5][0-9])+\s([a-zA-Z]+)"

    def __init__(self, time_argument: str):

        self.time_argument = time_argument

        # time format check
        if self.is_12_hour_format(self.time_argument):
            self.time_format = 12

        elif self.is_24_hour_format(self.time_argument):
            self.time_format = 24

        else:
            TimeZones.incorrect_time_format()

        #  if we are running from Alfred, use the correct path
        file_path = "./tz/" if os.getenv("alfred_version") else "./"

        with open(file_path + "preferences.json") as f:
            self.preferences = json.load(f)

        with open(file_path + "timezones.json") as f:
            self.timezones = json.load(f)

    def is_12_hour_format(self, input: str):
        pattern = re.compile(self.PATTERN_12)
        return re.match(pattern, input)

    def is_24_hour_format(self, input: str):
        pattern = re.compile(self.PATTERN_24)
        return re.match(pattern, input)

    def get_regex_groups(self):
        pattern = re.compile(
            self.PATTERN_12 if self.time_format == 12 else self.PATTERN_24)
        matches = re.match(pattern, self.time_argument)
        groups = matches.groups()
        return groups

    def display_24_hour_times(self, groups: tuple):
        hour = int(groups[0])
        minute = int(groups[1])
        submitted_timezone = groups[2]

        year = pendulum.now().year
        month = pendulum.now().month
        day = pendulum.now().day

        try:
            timezone: dict | None = next(filter(lambda tz: tz['abbreveation'].upper()
                                                == submitted_timezone.upper(), self.timezones), None)
            time = pendulum.datetime(year=year, month=month,
                                     day=day, hour=hour, minute=minute, tz=timezone['timezone'])
        except (IndexError, TypeError):
            TimeZones.unknown_timezone(submitted_timezone)

        return self.get_times(time=time, submitted_timezone=submitted_timezone)

    def display_12_hour_times(self, groups: tuple):
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

        try:
            timezone: dict = next(filter(lambda tz: tz['abbreveation'].lower()
                                         == submitted_timezone.lower(), self.timezones), None)
            time = pendulum.datetime(year=year, month=month,
                                     day=day, hour=hour, minute=minute, tz=timezone['timezone'])
        except (IndexError, TypeError):
            TimeZones.unknown_timezone(submitted_timezone)

        return self.get_times(time=time, submitted_timezone=submitted_timezone)

    def get_times(self, time: DateTime, submitted_timezone: str) -> dict:

        # which timezone does the user want
        time_format = "h:mmA" if self.time_format == 12 else "HH:mm"
        # store timezones already created
        created_times = set()
        # store timezones to be returned
        times_to_return = []

        for preference in self.preferences['timezones_to_display']:
            # if requested timezone is in available timezones skip
            if preference.upper() not in self.preferences["available_timezones"]:
                continue

            # if the timezone is the same as the user's submitted timezone, skip it
            elif preference.upper() == submitted_timezone.upper():
                continue

            # if the timezone is already created, skip it
            elif preference.upper() in created_times:
                continue

            timezone: dict = next(filter(
                lambda tz: tz['abbreveation'].upper() == preference.upper(), self.timezones), None)

            time_to_return = f"{time.in_timezone(timezone['timezone']).format(time_format, locale='en')} {timezone['abbreveation']}"
            created_times.add(timezone['abbreveation'].upper())
            times_to_return.append(
                {"title": time_to_return, "arg": time_to_return, 'icon': {'path': 'images/clock.png'}})

        return {'items': list(times_to_return)}

    def display_times(self):
        if self.time_format == None:
            return TimeZones.incorrect_time_format()

        groups = self.get_regex_groups()
        times = self.display_12_hour_times(
            groups) if self.time_format == 12 else self.display_24_hour_times(groups)
        print(json.dumps(times, indent=4, ensure_ascii=False))
        sys.exit(0)

    @staticmethod
    def incorrect_time_format() -> str:
        print(json.dumps(
            {'items': [{'title': "Incorrect time format", "subtitle": "example: 10:34am gmt or 18:30 pst"}]}, indent=4))
        sys.exit(0)

    @staticmethod
    def unknown_timezone(timezone: str) -> str:
        print(json.dumps(
            {'items': [{'title': "Unknown timezone", "subtitle": f"{timezone} is not a known timezone"}]}, indent=4))
        sys.exit(0)


def list_all_timezones():
    #  if we are running from Alfred, use the correct path
    file_path = "./tz/" if os.getenv("alfred_version") else "./"

    with open(file_path + "./preferences.json") as p:
        timezones = json.load(p)

    outout = map(
        lambda tz: {'title': tz}, timezones['available_timezones'])
    print(json.dumps({'items': list(outout)}, indent=4))
    sys.exit(0)


if __name__ == "__main__":

    if len(sys.argv) != 2:
        TimeZones.incorrect_time_format()

    if sys.argv[1] == "list":
        list_all_timezones()

    timezone = TimeZones(time_argument=sys.argv[1])

    timezone.display_times()
