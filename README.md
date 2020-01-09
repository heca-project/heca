# heca

Hebrew calendar written in rust. It converts from Hebrew to Gregorian and back and list Jewish holidays.

[![Crates.io](https://img.shields.io/crates/v/heca.svg)](https://crates.io/crates/heca)
[![Build Status](https://travis-ci.org/heca-project/heca.svg?branch=master)](https://travis-ci.org/heca-project/heca)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/heca-project/heca.svg)](https://isitmaintained.com/project/heca-project/heca "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/heca-project/heca.svg)](https://isitmaintained.com/project/heca-project/heca "Percentage of issues still open")
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

### Direct
You can download Linux (statically linked) and macos executables (untested) from the [release page](https://github.com/heca-project/heca/releases).

Then run
```
$ mv ~/Downloads/heca-$VERSION-$TARGET /usr/local/bin/heca
$ chmod +x /usr/local/bin/heca
$ heca
```
### Arch AUR
```
$ yaourt -S heca
```

### Cargo
If you have cargo installed, you can run:

```
$ cargo install heca
```

## Usage
    
    heca [OPTIONS] [SUBCOMMAND]


### Options

1. `--config`: Sets the config file. See the Config section for more information. If not set, it tries to read `$XDG_CONFIG_HOME/heca/config.toml`).
2. `--language`: Sets the output language. The options are Hebrew (he_IL) or English (en_US). If not set, it tries to pick up your languages from the `LANG` environment variable. If `LANG` isn't set (or is set to something not `he_IL`), it defaults to English.
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
 
   Can also be configured through setting `HECA_NOSORT=1`
2. `--years <AmountYears>`: Generate events for n years. Defaults to 1.
3. `--show <Events>`: What events to list. Possible values are:
     1. `yom-tov` - lists the main Yom Tovs - Rosh Hashana, Yom Kippur, Pesach, Shavuos and Sukkos.
     2. `shabbos` - lists the weekly Torah portion.
     3. `special-parshas` - lists the four special Torah portions read in the winter.
     4. `chol` - Shows weekdays that have special Torah readings - includes Shushan Purim.
     5. `minor-holidays` - Lag BaOmer, Pesach Sheni, and Erev Yom Tov.
     6. `omer` - Lists the Omer.
     7. `custom-holidays` - lists days in the config file.
     8. `daf-yomi` - lists the daily Daf Yomi.
     9. `yerushalmi-yomi` - lists the daily Yerushalmi Yomi.
     10. `rambam-3-chapters` - lists the daily Rambam (3 chapters a day).
     11. `rambam-1-chapter` - lists the daily Rambam (1 chapter a day).
     12. `israeli-holidays` - lists the Israeli holidays that hebcal displays (Yom HaAliyah, Sigd, Yom HaShoah, Yom HaZikaron, Yom HaAtzmaut, and Yom Yerushalayim).
     13. `chabad-holidays` - lists the days when Chabad doesn't say Tachanun (10 Kislev, 19/20 Kislev, and 12/13 Tammuz).
     14. `shabbos-mevarchim` - lists the Shabbos Mevorchim of the upcoming month. It also outputs the time of the molad (new moon).

     The default is `yom-tov`.
4. `--location`: Selects if you're looking for an Israeli calendar or Chu"l calendar. Options are "Chul" or "Israel". It defaults to Chul unless the language is Hebrew, in which case it defaults to Israel. Can also be configured through `HECA_LOCATION`.
5. `--type`: Force conversion from type T, where T is either "hebrew" (then date must be written as '5/אדרא/5779'), "gregorian" (where the date must be written as '1996/12/19'), or fuzzy (assumes Hebrew if `year` is above 4000, Gregorian otherwise. It also tries to be fuzzy in Hebrew month spelling and order). Defaults to `fuzzy`.
 
    Can also be configured through `HECA_YEAR_TYPE`.
    
6. `--exact-days`: There's an [argument](https://www.halachipedia.com/index.php?title=Yom_HaAtzmaut#cite_ref-6) between the Rabbanut, and Rabbis Aharon Soloveitchik and Hershel Schachter. The Rabbanut says that if these days fall out around Shabbos, we should move them so that it won't lead to people breaking Shabbos, 
                   while Rabbis Soloveitchik and Hershel Schachter said that one in America, one should always celebrate it on the given day. This option overrides the default Psak of the Rabbanut.
                    
      Can also be configured through `HECA_EXACT_DAYS`.    

## Config file

The config is a TOML file, with several options:

1. days - An array. Can be made out of:

   a. _Deprecated_: A three-element array. The first is the Hebrew day, the second is the string to output when pretty printing and, and 
   the third is the string to output when JSON printing). If the date doesn't exist in a certain year (For example, 
   not all years have an Adar Beis, 30th of Cheshvan or 30th of Kislev), that date is ignored. 

   b. An object of: `date`, `title`, `json`, and (optionally) `ifNotExists`. If `date` doesn't exist,
    then print it on all dates in `ifNotExist`. 

2. `language` - The default language (options: `en_US` or `he_IL`).
3. `location` - The default location (options: `Chul` or `Israel`).
4. `exact-days` - See above in the arguments section. (option: `true` or `false`).

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
language = "en_US"
exact-days = true
```


## Examples

### What's the difference between Israeli Torah reading and Diaspora?

```
$ diff  <(./target/release/heca list 2019 --show shabbos) <(./target/release/heca list 2019 --location Israel --show shabbos)

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
$ for i in `seq 5779 5900`; do echo "$i-$(date -d $(./target/release/heca --print json list $i --show minor-holidays |jq '.|.[] | select(.name == "ErevPesach") | .day' | tr -d \") '+%a')" ; done | grep "Fri"

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

### When will there be a Siuyum of both Rambam and Daf Yomi on the same day?

```
$ ./target/release/heca --print=json list 1985 --years 5000 --show daf-yomi |jq '.[] | select(.topic.masechta | contains("Berakhot")) | select(.topic.daf | contains(2)) | .day' > /tmp/siyum_daf_yomi
$ ./target/release/heca --print=json list 1985 --years 5000 --show rambam-3-chapters |jq '.[] | select(.topic[0].halacha | contains("Transmission")) | select(.topic[0].chapter | contains(1)) | .day' > /tmp/siyum_rambam_3_chapters
$ comm -12 /tmp/siyum_daf_yomi /tmp/siyum_rambam_3_chapters

"3155-08-25T18:00:00Z"
"5671-11-11T18:00:00Z"
``` 

So we'll have to wait a while for an Achdus Siyum.

### When will the Molad be on a round number?
```
$ ./target/release/heca --print=json list --show=shabbos-mevarchim 1020 --years 5000 |jq '. |.[] | select(.molad.minute ==0) | select(.molad.chalakim == 0) | .day'

"2092-02-01T18:00:00Z"
"2179-06-04T18:00:00Z"
"2267-09-13T18:00:00Z"
"2354-01-22T18:00:00Z"
"2441-05-17T18:00:00Z"
```
So, only in seventy years.
### When will the Molad be on a _really_ round number?

```
$ ./target/release/heca --print=json list --show=shabbos-mevarchim 1020 --years 5000 |jq '. |.[]| select(.molad.hour == 0) | select(.molad.minute ==0) | select(.molad.chalakim == 0) | .day'

"1830-02-19T18:00:00Z"
"3925-10-23T18:00:00Z"
```

We'll have to wait for a _while_ for the Molad to fall out exactly on midnight.

## Benchmarks

In my _totally not scientific benchmarks_:

```
$ ./benchmark/bench.sh

```
| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `taskset -ac 0-3 /tmp/heca/release/heca --print=regular list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas --no-sort` | 0.474 | 0.471 | 0.483 | 1.00 |
| `taskset -ac 0-3 /tmp/heca/release/heca --print=regular list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas` | 0.559 | 0.558 | 0.562 | 1.00 |
| `taskset -ac 1 /tmp/heca/release/heca --print=regular list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas --no-sort` | 0.631 | 0.627 | 0.637 | 1.33 ± 0.01 |
| `taskset -ac 1 /tmp/heca/release/heca --print=regular list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas` | 0.826 | 0.820 | 0.839 | 1.48 |
| `taskset -ac 0-3 /tmp/heca/release/heca --print=json list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas --no-sort` | 0.915 | 0.909 | 0.927 | 1.00 |
| `taskset -ac 0-3 /tmp/heca/release/heca --print=json list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas` | 1.001 | 997.1 | 1012.4 | 1.00 |
| `taskset -ac 1 /tmp/heca/release/heca --print=json list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas --no-sort` | 1.072 | 1068 | 1.078 | 1.17 ± 0.01 |
| `taskset -ac 1 /tmp/heca/release/heca --print=json list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas` | 1.268 | 1260.8 | 1276.0 | 1.27 ± 0.01 |

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `taskset -ac 1 hebcal 3766 --years 17000` | 1.012 | 1.008 | 1.030 | 1.00 |
| `taskset -ac 0-3 hebcal 3766 --years 17000` | 1.012 | 1.008 | 1.021 | 1.00 |


## Versioning

We use [SemVer](http://semver.org/) for versioning of JSON output (although we may add new holidays in minor releases). All other output may change at any time.

## License 

* Source is licensed under the MIT license.
* Binary is licensed under the MIT + Apache-2 license.