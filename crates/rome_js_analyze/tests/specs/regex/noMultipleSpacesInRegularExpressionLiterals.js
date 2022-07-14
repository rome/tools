// valid
/foo {2}bar/;
/foo bar baz/;
/foo bar\tbaz/;
/foo /;
// invalid
/   /;
/  foo/;
/foo   /;
/foo  bar/;
/foo   bar    baz/;
/foo [ba]r  b(a|z)/;
