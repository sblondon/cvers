# cvers
Compare VERSion numbers

Fonctionnement actuel :

```
$ cvers 1.0 1.1
<
```

résultat possible :  <, =,  >
                    -1, 0, 1



vnum : Version NUMbers (ou juste `vn`?)

```
$ vnum --compare '1.0' '<' '1.1'
1
```

--exit-value option

```
$ vnum compare '1.0' '1.1'

$ vnum eval '1.1' '<' '1.2' --exit-value
```