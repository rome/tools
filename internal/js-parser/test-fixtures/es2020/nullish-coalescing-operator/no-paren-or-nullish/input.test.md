# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2020 > nullish-coalescing-operator > no-paren-or-nullish`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSLogicalExpression {
				operator: "??"
				left: JSLogicalExpression {
					operator: "||"
					left: JSReferenceIdentifier {
						name: "h"
						loc: SourceLocation es2020/nullish-coalescing-operator/no-paren-or-nullish/input.js 1:0-1:1 (h)
					}
					right: JSReferenceIdentifier {
						name: "i"
						loc: SourceLocation es2020/nullish-coalescing-operator/no-paren-or-nullish/input.js 1:5-1:6 (i)
					}
					loc: SourceLocation es2020/nullish-coalescing-operator/no-paren-or-nullish/input.js 1:0-1:6
				}
				right: JSReferenceIdentifier {
					name: "j"
					loc: SourceLocation es2020/nullish-coalescing-operator/no-paren-or-nullish/input.js 1:10-1:11 (j)
				}
				loc: SourceLocation es2020/nullish-coalescing-operator/no-paren-or-nullish/input.js 1:0-1:11
			}
			loc: SourceLocation es2020/nullish-coalescing-operator/no-paren-or-nullish/input.js 1:0-1:12
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2020/nullish-coalescing-operator/no-paren-or-nullish/input.js>
	loc: SourceLocation es2020/nullish-coalescing-operator/no-paren-or-nullish/input.js 1:0-2:0
}
```

### `diagnostics`

```

```
