---
title: Lint Rule NoRedundantUseStrict
parent: lint/rules/index
---

# NoRedundantUseStrict (since v11.0.0)

Prevents from having redundant "use strict"

## Examples

### Invalid

"use strict";
function foo() {
"use strict";
}

### valid

"use strict";

function foo() {

}

### valid

function foo() {
"use strict";
}
function bar() {
"use strict";
}

