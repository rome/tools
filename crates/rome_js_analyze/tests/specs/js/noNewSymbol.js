// valid
var bar = Symbol('bar');
function baz() {
    function Symbol() { }
    new Symbol();
}
// invalid
var foo = new Symbol('foo');
var foo2 = new Symbol();
var lorem = new Symbol() // comment