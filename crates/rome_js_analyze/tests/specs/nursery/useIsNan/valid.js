var x = NaN;
isNaN(NaN) === true;
isNaN(123) !== true;
Number.isNaN(NaN) === true;
Number.isNaN(123) !== true;
foo(NaN + 1);
foo(1 + NaN);
foo(NaN - 1)
foo(1 - NaN)
foo(NaN * 2)
foo(2 * NaN)
foo(NaN / 2)
foo(2 / NaN)
var x; if (x = NaN) { }
var x = Number.NaN;
isNaN(Number.NaN) === true;
Number.isNaN(Number.NaN) === true;
foo(Number.NaN + 1);
foo(1 + Number.NaN);
foo(Number.NaN - 1)
foo(1 - Number.NaN)
foo(Number.NaN * 2)
foo(2 * Number.NaN)
foo(Number.NaN / 2)
foo(2 / Number.NaN)
var x; if (x = Number.NaN) { }
x === Number[NaN];

// switch-case
switch(foo) {}
switch(foo) { case bar: NaN; }
switch(foo) { default: NaN; }
switch(Nan) {}
switch('NaN') { default: break; }
switch(foo(NaN)) {}
switch(foo.NaN) {}
switch(foo) { case Nan: break }
switch(foo) { case 'NaN': break }
switch(foo) { case foo(NaN): break }
switch(foo) { case foo.NaN: break }
switch(foo) { case bar: break; case 1: break; default: break; }
switch(foo) { case bar: Number.NaN; }
switch(foo) { default: Number.NaN; }
switch(Number.Nan) {}
switch('Number.NaN') { default: break; }
switch(foo(Number.NaN)) {}
switch(foo.Number.NaN) {}
switch(foo) { case Number.Nan: break }
switch(foo) { case 'Number.NaN': break }
switch(foo) { case foo(Number.NaN): break }
switch(foo) { case foo.Number.NaN: break }

