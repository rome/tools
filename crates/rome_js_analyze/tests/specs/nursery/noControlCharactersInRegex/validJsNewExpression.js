var regex1 = new RegExp("x1f");
var regex2 = new RegExp("[");
new (function foo() {})("\\x1f");
var regex3 = new RegExp("\\u{20}", "u");
var regex4 = new RegExp("\\u{1F}");
var regex5 = new RegExp("\\u{1F}", "g");
var regex6 = new RegExp("\\u{1F}", uflags);
