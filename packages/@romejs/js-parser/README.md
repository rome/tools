Last updated tests: July 6, 2020

```
babel$ git log --since="April 2019" --name-only --pretty=format: | grep babel-parser/test/fixtures | grep -v flow | grep -v scope | grep -v placeholders | grep -v output.json | sort | uniq >files.txt
```
