// shouldn't autofix if the call doesn't have exactly two arguments
Math.pow()
Math.pow(a)
Math.pow(a, b, c)
Math.pow(a, b, c, d)

// shouldn't autofix if any of the arguments is spread
Math.pow(...a)
Math.pow(...a, b)
Math.pow(a, ...b)
Math.pow(a, b, ...c)

// shouldn't autofix if that would remove comments
/* comment */Math.pow(a, b)
Math.pow(/**/a, b)
Math.pow(a, b/**/)
Math.pow(a, b)/* comment */;
Math.pow(a, b)// comment;
Math.pow(/**/a/**/, /**/b/**/)
