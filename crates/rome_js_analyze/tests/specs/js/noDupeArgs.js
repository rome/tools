// valid
function a([ , a]) {}

// invalid
function b(a, b, b) {}

function c(a, a, a) {}

function d(a, b, a) {}

function e(a, b, a, b) {}

var f = function(a, b, b) {}

var g = function(a, a, a) {}

var h = function(a, b, a) {}