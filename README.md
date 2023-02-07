# cvers
Compare VERSion numbers

Two ways to call `cvers`:

```
$ cvers compare version_a version_b
$ cvers assert version_a operator version_b
```

Every parameters are mandatory.


## `compare` parameter

```
$ cvers compare 1.0 1.1
<
```

print the result on standard output.
3 characters can be displayed:
 - `<`: version_a is lesser than version_b
 - `=`: version_a equals version_b
 - `>`: version_a is greater than version_b

In previous example, version_a is 1.0 and version_b is 1.1.


## `assert` parameter

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

## Use-case example

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

## Versioning schemes

 - [semantic versioning](https://semver.org/)
 - [debian](https://www.debian.org/doc/debian-policy/ch-controlfields.html#version)
 - [python](https://peps.python.org/pep-0440/)


## Structure parser

```
[epoch][separator][main chunk][char touches main chunk][suffix]
```

`suffix` could be prerelease or postrelease data.


## Other solutions

 - shell script like [this one](https://stackoverflow.com/questions/4023830/how-to-compare-two-strings-in-dot-separated-version-format-in-bash) or [here](https://unix.stackexchange.com/questions/285924/how-to-compare-a-programs-version-in-a-shell-script/285928)
 - `dpkg --compare-versions` (only on debian and derivatives)
 - `sort -V`


## Rename project?

 - vnum for Version NUMbers
 - vrsns
 - versn
