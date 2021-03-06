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
 - `<`: version_a is lesser than version_b
 - `=`: version_a equals version_b
 - `>`: version_a is greater than version_b


assert
------

```
$ cvers assert '1.1' '<<' '1.2'
$ echo $?
0
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


UseCase

```sh
current_version=$(/usr/sbin/logrotate 2>&1 | head -n 1 | cut -d' ' -f 2)
if cvers assert "$current_version" "<<" "3.9.2"
then
    /usr/sbin/logrotate /path/to/file.conf
else
    # 3.9.2 introduce -l for logging
    /usr/sbin/logrotate /path/to/file.conf -l "/path/to/log/dir"
fi
```



autre nom possible :
 - vnum pour Version NUMbers
 - vrsns
 - versn