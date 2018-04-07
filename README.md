## how to use mint

### invoking mint

`mint TMPLPATH [NAME=VAL...]`

Variable names can't start with `!`, and they can't contain `}}`, `=`, or
newlines. Variable values can contain anything at all.

### template files

A template file is made up of tags and arbitrary text. A tag looks like
`{{name}}` and is replaced with the value of the variable `name`. `{{!` becomes
a literal `{{`. Currently, template files and variable definitions must use
UTF-8.

### example

`template.txt:`
```none
{{!foo}} renders to {{foo}}, and {{!! renders to {{!.
It doesn't matter if {{!bar}} is defined, because we don't use it.
```

`mint template.txt foo=cat bar=sock`

Output:
```none
{{foo}} renders to cat, and {{! renders to {{.
It doesn't matter if {{bar}} is defined, because we don't use it.
```


## the name

"Mint" stands for "MINimal Template engine". I'm not good at naming things.
