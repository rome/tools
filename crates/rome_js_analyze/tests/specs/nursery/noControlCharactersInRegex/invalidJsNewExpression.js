var regex1 = new RegExp("\\x1f\\x1e");
var regex2 = new RegExp("\\x1fFOO\\x00");
var regex3 = new RegExp("FOO\\x1fFOO\\x1f");
var regex4 = new RegExp("\\x1f");
var regex5 = new RegExp("\\u001F", flags);
var regex6 = new RegExp("\\u{1111}*\\x1F", "u");
var regex7 = new RegExp("\\u{1F}", "u");
var regex8 = new RegExp("\\u{1F}", "gui");