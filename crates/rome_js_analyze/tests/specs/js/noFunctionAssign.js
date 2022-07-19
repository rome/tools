function foo() { };
foo = bar;

function foo2() { foo2 = bar; }

foo3 = bar;
function foo3() { };

[foo4] = bar;
function foo4() { };

({ x: foo5 = 0 } = bar);
function foo5() { };

function foo6() { [foo6] = bar; }

(function () { ({ x: foo7 = 0 } = bar); function foo7() { }; })();

// Valid
function foo8() { var foo8 = bar; }
function foo9(foo9) { foo9 = bar; }
function foo10() { var foo10; foo10 = bar; }
var foo11 = () => { }; foo11 = bar;
var foo12 = function () { }; foo12 = bar;
var foo13 = function () { foo13 = bar; };