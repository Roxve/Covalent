# .grammar file desc
**sorry for it being poorly written, one must learn how to write in a good way to be able to write in a good way and for that, one must write**

each rule is defined like this
```
rule -> ...;
```
`rule` is the rule name ... is the defintion end the definiton with ;

defintion may contain other rules without ""
```
rule -> other_rule;
```
or terminal "tokens" or 'tokens'
```
rule -> "rule";
```

```
rule -> rule1 | rule0;
```
means that rule can be ethier rule1 or rule0
```
rule -> ("token1" "token0") | (rule2);
```
() groups a definition
this means token1 then token0, or (insert rule2 here)

```
rule -> rule2?;
```
? means that rule2 may appear one or zero times
```
rule -> rule2*;
```
* means that rule2 may appear zero or more times
```
rule -> rule2+;
```
+ means that rule2 may appear one or more times
