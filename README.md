# Simple Grade Calculator #
[![Build Status](https://travis-ci.com/TobsCore/grades-rs.svg?branch=master)](https://travis-ci.com/TobsCore/grades-rs)

This tool is a simple grade average calulcation program, that is written in Rust. The logic for grade calculations, are encapsulated as a library and therefore be used by other applications. It takes a list of graded to calculated the average.

## Usage ##

The application can read the input grades either as program arguments, or from STDIN. The usage is shown below:

```bash
grade-avg 1.3 2.0
```

Another way to use the tool would be to create a file and use the file's contents as the input grades. Imagine a file `~/grades.txt` with the contents:

```
1.3
2.0
1.7
```

The average for these grades can be calculcated as follow:

```bash
cat ~/grades.txt | grade-avg
```