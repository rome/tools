// SCRIPT

"use new"

let a = 10;

"use strict"; // not a directive

function test() {
	"use strict";

	let a = 10;

	"use strict";
}

(function () {
	"use strict";

	let a = 10;

	"use strict"; // not a directive
});

let b = () => {
	"use strict";

	let a = 10;

	"use strict";
}

{
	"use strict"; // not a directive
}
