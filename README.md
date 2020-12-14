# cvers
Compare VERSion numbers

Two ways to call `cvers`:

```
$ cvers compare version_a version_b
$ cvers assert version_a operator version_b
```

Every parameters are mandatory.


compare
-------

```
$ cvers compare 1.0 1.1
<
```

print the result on standard output.
3 characters can be displayed:
 - `<`
 - `=`
 - `>`


assert
------

```
$ cvers assert '1.1' '<<' '1.2'
```

There are no output.
The exit value are 0 (if assert is true) or 1 (if assert is false).

Accepted operators are:
 - `<<`: strictly less
 - `<=`: less or equal
 - `==`: equal
 - `>=`: greater or equal
 - `>>`: strictly greater

The behaviour is equivalent to `dpkg --compare-versions`.


autre nom possible :
 - vnum pour Version NUMbers
 - vrsns
 - versn