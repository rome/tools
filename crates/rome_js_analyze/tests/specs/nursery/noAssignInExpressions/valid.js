{
	let a;
	a = 1;
}

{
	let a, b;
	a = b = 1;
}

{
	let a, b;
	(a = 1), (b = 2);
}

{
	let a;
	label: a = 1;
}

{
	// This is not an assignment
	const { a = 1 } = {};
}

// This is not an assignment
function f(a = 1) {}

for (let a = 0; a < 10; a += 2) {}

for (let a = 0, b = 0; a < 10; a += 2, b += 3) {}

// Assign in initialization
for (a = 0; a < 10; a += 2) {}

for (;;) {}

if (x == 0) {
	let b = 1;
}

while (x < 5) {
	x = x + 1;
}
