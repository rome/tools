# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > await-async-function-expression-name`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSFunctionExpression {
				id: JSBindingIdentifier {
					name: "await"
					loc: SourceLocation es2017/async-functions/await-async-function-expression-name/input.js 1:16-1:21 (await)
				}
				body: JSBlockStatement {
					body: []
					directives: []
					loc: SourceLocation es2017/async-functions/await-async-function-expression-name/input.js 1:24-1:26
				}
				head: JSFunctionHead {
					async: true
					generator: false
					hasHoistedVars: false
					params: []
					loc: SourceLocation es2017/async-functions/await-async-function-expression-name/input.js 1:21-1:23
				}
				loc: SourceLocation es2017/async-functions/await-async-function-expression-name/input.js 1:1-1:26
			}
			loc: SourceLocation es2017/async-functions/await-async-function-expression-name/input.js 1:0-1:28
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "Can not use 'await' as identifier inside an async function"}
			}
			location: {
				language: "js"
				path: UIDPath<es2017/async-functions/await-async-function-expression-name/input.js>
				end: Position 1:21
				start: Position 1:16
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2017/async-functions/await-async-function-expression-name/input.js>
	loc: SourceLocation es2017/async-functions/await-async-function-expression-name/input.js 1:0-1:28
}
```

### `diagnostics`

```

 es2017/async-functions/await-async-function-expression-name/input.js:1:16 parse(js) ━━━━━━━━━━━━━━━

  ✖ Can not use 'await' as identifier inside an async function

    (async function await() {});
                    ^^^^^


```
