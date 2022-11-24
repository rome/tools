// tokens that cannot be adjacent
a+Math.pow(++b, c);
(a)+(Math).pow((++b), c);

// not supported yet
//Math.pow(a, b)in c
//Math.pow(a, (b))in (c)
//a+Math.pow(++b, c)in d
//a+Math.pow( ++b, c )in d

// tokens that cannot be adjacent, but the autofix inserts parens required for precedence
+Math.pow(++a, b)
Math.pow(a, b) + Math.pow(c, d)
Math.pow(Math.pow(a, b), Math.pow(c, d))
Math.pow(a, b)**Math.pow(c, d)
