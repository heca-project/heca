# heca
--------
Hebrew calendar written in rust. As of now, it can convert from Hebrew to Gregorian and back. 

### Installation

If you have cargo installed, you can run:

```
$ cargo install heca
```

### Usage

The only command that currently works is convert, which takes a single argument of the day you want to convert.

* To convert from Gregorian: You can use any argument supported by [chrono-english](https://github.com/stevedonovan/chrono-english).

  Example:

  ```

  $ heca convert now
  2019-02-07 01:00:00 UTC -> 2 Adar 1 5779
  
  ```
  ```

  $ heca convert 5/4/2003
  2003-05-04 00:00:00 UTC -> 2 Iyar 5763
  
  ```

If you want to convert from Hebrew, you can input it in any format. The choice of month names is "Tishrei", "Cheshvan", "Kislev", Teves, Shvat, Adar, Adar1, Adar2, Nissan, Iyar, Sivan, Tammuz, Av, Elul.

For example, to get the Gregorian date for the 24th of Teves, you can run:

  ```
  
  $ heca convert "24/Teves/5779"
  2018-12-31 18:00:00 UTC

  ```

or 

  ```
  
  $heca convert "Teves 24 5779"
  2018-12-31 18:00:00 UTC

  ```

or even 

  ```
  
  $heca convert "5779_24_Teves"
  2018-12-31 18:00:00 UTC

  ```


Licensed under the MIT license.
