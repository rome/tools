// valid
function a(a, b, c) {}
var j = function (j, b, c) {};
function k({ k, b }, { c, d }) {}
function l([, l]) {}
function foo([[a, b], [c, d]]) {}
function test(a  = function(a) {}) {}
// invalid
function b(a, b, b) {}

function c(a, a, a) {}

function d(a, b, a) {}

function e(a, b, a, b) {}

var f = function(a, b, b) {}

var g = function(a, a, a) {}

var h = function(a, b, a) {}

export default function (a, b, a, a) {}
