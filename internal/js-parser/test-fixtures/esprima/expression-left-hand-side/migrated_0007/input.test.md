# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > expression-left-hand-side > migrated_0007`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSCallExpression {
				arguments: []
				callee: JSMemberExpression {
					object: JSNewExpression {
						arguments: []
						callee: JSReferenceIdentifier {
							name: "foo"
							loc: SourceLocation esprima/expression-left-hand-side/migrated_0007/input.js 1:6-1:9 (foo)
						}
						loc: SourceLocation esprima/expression-left-hand-side/migrated_0007/input.js 1:2-1:9
					}
					property: JSStaticMemberProperty {
						value: JSIdentifier {
							name: "bar"
							loc: SourceLocation esprima/expression-left-hand-side/migrated_0007/input.js 1:11-1:14 (bar)
						}
						loc: SourceLocation esprima/expression-left-hand-side/migrated_0007/input.js 1:11-1:14 (bar)
					}
					loc: SourceLocation esprima/expression-left-hand-side/migrated_0007/input.js 1:0-1:14
				}
				loc: SourceLocation esprima/expression-left-hand-side/migrated_0007/input.js 1:0-1:16
			}
			loc: SourceLocation esprima/expression-left-hand-side/migrated_0007/input.js 1:0-1:16
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/expression-left-hand-side/migrated_0007/input.js>
	loc: SourceLocation esprima/expression-left-hand-side/migrated_0007/input.js 1:0-2:0
}
```

### `diagnostics`

```

```
