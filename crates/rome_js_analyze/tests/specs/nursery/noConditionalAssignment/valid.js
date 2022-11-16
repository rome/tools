var x = 0; if (x == 0) { var b = 1; }
var x = 0; if (x == 0) { var b = 1; }
var x = 5; while (x < 5) { x = x + 1; }
if ((someNode = someNode.parentNode) !== null) { }
if ((someNode = someNode.parentNode) !== null) { }
if ((a = b));
while ((a = b));
do {} while ((a = b));
for (;(a = b););
for (;;) {}
if (someNode || (someNode = parentNode)) { }
while (someNode || (someNode = parentNode)) { }
do { } while (someNode || (someNode = parentNode));
for (;someNode || (someNode = parentNode););
if ((function(node) { return node = parentNode; })(someNode)) { }
if ((function(node) { return node = parentNode; })(someNode)) { }
if ((node => node = parentNode)(someNode)) { }
if ((node => node = parentNode)(someNode)) { }
if (function(node) { return node = parentNode; }) { }
if (function(node) { return node = parentNode; }) { }
x = 0;
var x; var b = (x === 0) ? 1 : 0;
switch (foo) { case a = b: bar(); }
switch (foo) { case a = b: bar(); }
switch (foo) { case baz + (a = b): bar(); }
