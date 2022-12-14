var pattern1 = /\x00/;
var pattern2 = /\x0C/;
var pattern3 = /\x1F/;
var pattern4 = /\u000C/;
var pattern5 = /\u{C}/u;
var pattern6 = /\\\x1f\\x1e/;
var pattern7 = /\\\x1fFOO\\x00/;
var pattern8 = /FOO\\\x1fFOO\\x1f/;
var pattern9 = /(?<a>\\x1f)/;
var pattern10 = /(?<\u{1d49c}>.)\x1f/;
var pattern11 = /\u{1111}*\x1F/u;
var pattern12 = /\u{1F}/u;
var pattern13 = /\u{1F}/gui;

