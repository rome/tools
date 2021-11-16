// SCRIPT

"use new"

let a = 10;

"use strict"; // not a directive

function test() {
	'use strict';

	let a = 10;

	'use strict'; // not a directive
}

(function () {
	"use strict";

	let a = 10;

	"use strict"; // not a directive
});

let b = () => {
	"use strict";

	let a = 10;

	"use strict";  // not a directive
}

{
	"use strict"; // not a directive
}
