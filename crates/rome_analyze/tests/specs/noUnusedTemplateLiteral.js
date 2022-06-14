// valid
var foo2 = `bar
has newline`;
var foo3 = `\"bar\"`
var foo4 = `'bar'`
var foo = `bar 'baz'`;

var foo = `back${x}tick`;
var foo = tag`backtick`;
var foo = `something 
else`;
//invalid
var foo = `bar`;
var foo1 = `bar `;
var foo = `back\rtick`;
var foo = `back\ntick`;
var foo = `back\u2028tick`
var foo = `back\u2029tick`;
var foo = `back\\\\\ntick`;
var foo = `\n`;
function foo() { `use strict`; foo(); }
var foo = `foo\\nbar`;
var foo = `foo\\\nbar`;
var foo = `foo\\\\\\\nbar`;