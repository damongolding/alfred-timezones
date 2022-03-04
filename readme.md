# Timezone workflow for Alfred

I mainly created this to learn how use Python but if you find it useful it's a win.

## Usage
Open the Alfred search bar and type `t` then add the time you wish to convert. You can use either 12 or 24 hour time.

**examples**
`t 1:30am gmt` or `t 1:30 gmt`
`t 10:00pm pst` or `t 21:00 pst`


### list all timezones
to view all timezones, just type `t list` in the Alfred search bar.


### Preferences
You can set your preferred timezone(s) by editing the `preferences.json` file.

There are two keys in the file: `timezones_to_display` and `available_timezones`.
* `timezones_to_display` is a list of timezones that will display in the results. They will be displayed in the order specified.
* `available_timezones` is a list of timezones that are available to be set as the preferred timezones.
