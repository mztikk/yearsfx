# yearsfx
yearsfx will append years in a given range to the end of every line piped into it.

The start year is a required argument with the end year being an optional second argument, if ommitted it will be the current year.


## Usage

```bash
yearsfx <start-year> [end-year]
```
```bash
echo "Hello World" | yearsfx 2015

Hello World2015
Hello World2016
Hello World2017
Hello World2018
Hello World2019
Hello World2020
Hello World2021
```
