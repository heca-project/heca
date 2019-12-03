# heca

Hebrew calendar written in rust. It converts from Hebrew to Gregorian and back and list Jewish holidays.

[![Crates.io](https://img.shields.io/crates/v/heca.svg)](https://crates.io/crates/heca)
[![Build Status](https://travis-ci.org/heca-project/heca.svg?branch=master)](https://travis-ci.org/heca-project/heca)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/heca-project/heca.svg)](https://isitmaintained.com/project/heca-project/heca "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/heca-project/heca.svg)](https://isitmaintained.com/project/heca-project/heca "Percentage of issues still open")
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

If you have cargo installed, you can run:

```
$ cargo install heca
```

## Usage
    
    heca [OPTIONS] [SUBCOMMAND]


### Options

1. `--config`: Sets the config file. See the Config section for more information. If not set, it tries to read `$XDG_CONFIG_HOME/heca/config.toml`).
2. `--language`: Sets the output language. The options are Hebrew (he_IL) or English (en_US). If not set, it tries to pick up your languages from the `LANG` environment variable. If `LANG` isn't set (or is set to something not `he_IL`), it outputs to English.
3. `--print`: Prints the result as JSON, regular or pretty-printed (is currently aliased to regular).

### Subcommands

#### Convert

    heca convert [OPTIONS] <Date>

##### Options
 
1. `--datefmt <DateFormat>`: Sets the date format (for Gregorian only): US or M for mm/dd/yyyy, UK or L for dd/mm/yyyy, ISO or B for yyyy/mm/dd. The default is ISO.
2. `--type <T>`: Force conversion from type T, where T is either "hebrew" (then date must be written as '5/אדרא/5779'), "gregorian" (where the date must be written as '1996/12/19'), or fuzzy (assumes Hebrew if `year` is above 4000, Gregorian otherwise. It also tries to be fuzzy in Hebrew month's spelling and order). Defaults to `fuzzy`.

#### List

    heca list [FLAGS] [OPTIONS] <Year>

##### Important point
 
Since the Jewish day starts at night-time, unlike most calendars, the days listed are the _starting_ day, not the ending day. So if the first Seder is on Friday night, I'll output that the first day of Pesach is Friday, not Shabbos.

##### Options

1. `--no-sort`: Doesn't sort output. This is useful if you're just looking for a certain date.
2. `--years <AmountYears>`: Generate events for n years. Defaults to 1.
3. `--show <Events>`: What events to list. Possible values are "yom-tov", "shabbos" (lists the Weekly parshas), "special-parshas" (lists the four special Parshas read in the winter), "chol" (Shows weekdays which have Torah readings, including Shushan Purim), "minor-holidays" (Lag BaOmer, Pesach Sheni, and Erev Yom Tov), "omer", "custom-holidays" (Days listed in the config file). Defaults to yom-tov.
4. `--location`: Selects if you're looking for an Israeli calendar or Chu"l calendar. Options are "Chul" or "Israel". It defaults to Chul unless the language is Hebrew, in which case it defaults to Israel.
5. `--type`: Force conversion from type T, where T is either "hebrew" (then date must be written as '5/אדרא/5779'), "gregorian" (where the date must be written as '1996/12/19'), or fuzzy (assumes Hebrew if `year` is above 4000, Gregorian otherwise. It also tries to be fuzzy in Hebrew month spelling and order). Defaults to `fuzzy`.

## Config file

The config is a TOML file, with several options:

1. days - An array. Can be made out of:

   a. A three-element array. The first is the Hebrew day, the second is the string to output when pretty printing and, and 
   the third is the string to output when JSON printing). If the date doesn't exist in a certain year (For example, 
   not all years have an Adar Beis, 30th of Cheshvan or 30th of Kislev), that date is ignored. 

   b. An object of: `date`, `title`, `json`, and (optionally) `ifNotExists`. If `date` doesn't exist,
    then print it on all dates in `ifNotExist`. 

2. language - The default language (options: `en_US` or `he_IL`).
3. location - The default location (options: `chul` or `israel`).

### Examples:
```
days = [
         ["10 שבט", "Yud Shvat (The Yom Hilula of the Previous Lubavitcher Rebbe)", "YudShvat"],
         ["1 אדר", "First of Adar", "1Adar"],
         ["1 אדרא", "First of Adar I", "1AdarI"],
         ["1 אדרב", "First of Adar II", "1AdarII"],
         ["30 כסלו", "30th of Kislev", "30Kislev"],
         ["30 חשוון", "30th of Cheshvan", "30 Cheshvan"],
       ]
```
```
days = [
   { date = "10 שבט", title = "Yud Shvat (The Yom Hilula of the Previous Lubavitcher Rebbe)", json = "YudShvat" },
   { date = "10 Adar2", ifNotExists = ["10 Adar"], title = "Yahrtzeit of Reb Moshe", json = "YahrtzeitRebMoshe" },
   { date = "30 Kislev", ifNotExists = ["29 Kislev", "1 Teves"], title = "This day doesn't always exist", json = "AnnoyingDay" },
   { date = "31 תשרי", ifNotExists = ["32 Adar2"], title = "Huh?", json = "HuhDay" }
]
```


## Examples

### What's the difference between Israeli Torah reading and Diaspora?

```
diff  <(./target/release/heca list 2019 --show shabbos) <(./target/release/heca list 2019 --location Israel --show shabbos)

16,29c16,30
< Night of 2019/5/3: Acharei Mos
< Night of 2019/5/10: Kedoshim
< Night of 2019/5/17: Emor
< Night of 2019/5/24: Behar
< Night of 2019/5/31: Bechukosai
< Night of 2019/6/7: Bamidbar
< Night of 2019/6/14: Naso
< Night of 2019/6/21: Behaaloscha
< Night of 2019/6/28: Shlach
< Night of 2019/7/5: Korach
< Night of 2019/7/12: Chukas
< Night of 2019/7/19: Balak
< Night of 2019/7/26: Pinchas
< Night of 2019/8/2: Matos/Maasei
---
> Night of 2019/4/26: Acharei Mos
> Night of 2019/5/3: Kedoshim
> Night of 2019/5/10: Emor
> Night of 2019/5/17: Behar
> Night of 2019/5/24: Bechukosai
> Night of 2019/5/31: Bamidbar
> Night of 2019/6/7: Naso
> Night of 2019/6/14: Behaaloscha
> Night of 2019/6/21: Shlach
> Night of 2019/6/28: Korach
> Night of 2019/7/5: Chukas
> Night of 2019/7/12: Balak
> Night of 2019/7/19: Pinchas
> Night of 2019/7/26: Matos
> Night of 2019/8/2: Maasei
```

### When's the next time the first Seder will be on a Friday night?

```
for i in `seq 5779 5900`; do echo "$i-$(date -d $(./target/release/heca --print json list $i --show minor-holidays |jq '.|.[] | select(.name == "ErevPesach") | .day' | tr -d \") '+%a')" ; done | grep "Fri"

5781-Fri
5785-Fri
5805-Fri
5808-Fri
5812-Fri
5832-Fri
5835-Fri
5839-Fri
5859-Fri
5863-Fri
5883-Fri
5890-Fri
```
## Benchmarks

In my _totally not scientific benchmarks_:

```
./benchmark/bench.sh

heca  | multithreaded   | unsorted   | 1.786
heca  | multithreaded   | sorted     | 2.189
heca  | singlethreaded  | unsorted   | 2.454
heca  | singlethreaded  | sorted     | 3.247
hebcal                               | 5.243
```


## Versioning

We use [SemVer](http://semver.org/) for versioning of JSON output (although we may add new holidays in minor releases). All other output may change at any time.

## License 

Licensed under the MIT license.
