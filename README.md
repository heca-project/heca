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

  ```
  ```

  $ heca convert 5/4/2003

  ```

  **NOTE**
  Unlike most calendars program, heca takes into account that the Hebrew day starts at nightfall and not midnight, but, as of now, it doesn't actually calculate nightfall. Rather, **it assumes that nightfall is at 6:00 PM**. So commands like `heca convert now` or `heca convert yesterday` may be off.

* If you want to convert from Hebrew, you can input it in any format, 

Licensed under the MIT license.
