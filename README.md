# heca
--------
Hebrew calendar written in rust. It converts from Hebrew to Gregorian and back, and list Jewish holidays.

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
2. `--language`: Sets the output language. The options are Hebrew (he_IL) or English (en_US). If not set, it tries to pick up your languages from the `LANG` environment variable. If `LANG` isn't set (or is set to something not `he_IL.UTF-8`), it outputs to English.
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
 
Since the Jewish day starts at night-time, unlike most calendars, the days listed are the _starting_ day, not the ending day. So if the first night of Pesach starts on Friday night, I'll output that day of the first day of Pesach as that Friday, not the next morning.

##### Options

1. `--no-sort`: Doesn't sort output. This is useful if you're just looking for a certain date.
2. `--years <AmountYears>`: Generate events for n years. Defaults to 1.
3. `--show <Events>`: What events to list. Possible values are "yom-tov", "shabbos" (lists the Weekly parshas), "special-parshas" (lists the four special Parshas read in the winter), "chol" (Shows weekdays which have Torah readings, including Shushan Purim), "minor-holidays" (Lag BaOmer, Pesach Sheni, and Erev Yom Tov), "omer", "custom-holidays" (Days listed in the config file). Defaults to yom-tov.
4. `--location`: Selects if you're looking for an Israeli calendar or Chu"l calendar. Options are "Chul" or "Israel". It defaults to Chul unless the language is Hebrew, in which case it defaults to Israel.
5. `--type`: Force conversion from type T, where T is either "hebrew" (then date must be written as '5/אדרא/5779'), "gregorian" (where the date must be written as '1996/12/19'), or fuzzy (assumes Hebrew if `year` is above 4000, Gregorian otherwise. It also tries to be fuzzy in Hebrew month spelling and order). Defaults to `fuzzy`.

## Config file

The config is a TOML file, with one option: 

1. days - An array made up of arrays of strings. It looks like this:

    
    days = [
      ["10 שבט", "Yud Shvat (The Yom Hilula of the Previous Lubavitcher Rebbe)", "YudShvat"],
      ["1 אדר", "First of Adar", "1Adar"],
      ["1 אדרא", "First of Adar I", "1AdarI"],
      ["1 אדרב", "First of Adar II", "1AdarII"],
      ["30 כסלו", "30th of Kislev", "30Kislev"],
      ["30 חשוון", "30th of Cheshvan", "30 Cheshvan"],
    ]

  The first element is the Hebrew date - day followed by month (in Hebrew), the second is the printable form, and the third is the JSON form. **Note** If the date doesn't exist in a given year (for example, if it's a leap year and you only put the date in Adar1, or if the Cheshvan of that year doesn't have a 30th day), **It doesn't output that day at all**. So if you want a day in Adar to be printed in all years, you have to add two days, one day in the regular Adar and one in Adar1 or Adar2, depending on your need).



## Examples

### What's the difference between Israeli Torah reading and Diaspora?

    diff  <(./target/release/heca list 2019 --show shabbos) <(./target/release/heca list 2019 --location Israel --show shabbos)

    16,29c16,30
    < 2019/5/3 Acharei Mos
    < 2019/5/10 Kedoshim
    < 2019/5/17 Emor
    < 2019/5/24 Behar
    < 2019/5/31 Bechukosai
    < 2019/6/7 Bamidbar
    < 2019/6/14 Naso
    < 2019/6/21 Behaaloscha
    < 2019/6/28 Shlach
    < 2019/7/5 Korach
    < 2019/7/12 Chukas
    < 2019/7/19 Balak
    < 2019/7/26 Pinchas
    < 2019/8/2 Matos/Maasei
    ---
    > 2019/4/26 Acharei Mos
    > 2019/5/3 Kedoshim
    > 2019/5/10 Emor
    > 2019/5/17 Behar
    > 2019/5/24 Bechukosai
    > 2019/5/31 Bamidbar
    > 2019/6/7 Naso
    > 2019/6/14 Behaaloscha
    > 2019/6/21 Shlach
    > 2019/6/28 Korach
    > 2019/7/5 Chukas
    > 2019/7/12 Balak
    > 2019/7/19 Pinchas
    > 2019/7/26 Matos
    > 2019/8/2 Maasei

### When's the next time Erev Pesach will be on a Shabbos?

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

## Benchmarks

In my _totally not scientific benchmarks_ done on my main computer (Intel quad-core):

    time for i in `seq 1 10`; do hebcal 5 --years 9999 -ors >/dev/null; done

    real  0m8.940s
    user  0m8.252s
    sys	  0m0.518s

    time for i in `seq 1 10`; do ./target/release/heca list 5 --years 9999 --show shabbos,special-parshas,chol,minor-holidays,omer >/dev/null ; done

    real  0m7.559s
    user  0m11.177s
    sys	  0m3.796s

## Versioning

We use [SemVer](http://semver.org/) for versioning of JSON output. All other output may change at any time.

Licensed under the MIT license.
