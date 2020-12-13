# cvers
Compare VERSion numbers

Fonctionnement actuel :

```
$ cvers compare 1.0 1.1
<
```

sortie possible (sur la sortie standard) :  <, =,  >



```
$ cvers assert '1.1' '<<' '1.2'
```

pas de sortie, valeur de retour 0 (assertion vraie) ou 1 (assertion fausse)

opérateurs de comparaison : '<<', '<=', '==', '>=', '>>'

comparable à `dpkg --compare-versions`


autre nom possible :
 - vnum pour Version NUMbers
 - vrsns
 - versn