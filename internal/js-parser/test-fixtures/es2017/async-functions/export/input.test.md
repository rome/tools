# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2017 > async-functions > export`

### `ast`

```javascript
JSRoot {
	body: [
		JSExportLocalDeclaration {
			exportKind: "value"
			declaration: JSFunctionDeclaration {
				id: JSBindingIdentifier {
					name: "foo"
					loc: SourceLocation es2017/async-functions/export/input.js 1:22-1:25 (foo)
				}
				body: JSBlockStatement {
					body: []
					directives: []
					loc: SourceLocation es2017/async-functions/export/input.js 1:28-1:30
				}
				head: JSFunctionHead {
					async: true
					generator: false
					hasHoistedVars: false
					params: []
					loc: SourceLocation es2017/async-functions/export/input.js 1:25-1:27
				}
				loc: SourceLocation es2017/async-functions/export/input.js 1:7-1:30
			}
			loc: SourceLocation es2017/async-functions/export/input.js 1:0-1:30
		}
		JSExportDefaultDeclaration {
			declaration: JSFunctionDeclaration {
				id: JSBindingIdentifier {
					name: "bar"
					loc: SourceLocation es2017/async-functions/export/input.js 2:30-2:33 (bar)
				}
				body: JSBlockStatement {
					body: []
					directives: []
					loc: SourceLocation es2017/async-functions/export/input.js 2:36-2:38
				}
				head: JSFunctionHead {
					async: true
					generator: false
					hasHoistedVars: false
					params: []
					loc: SourceLocation es2017/async-functions/export/input.js 2:33-2:35
				}
				loc: SourceLocation es2017/async-functions/export/input.js 2:15-2:38
			}
			loc: SourceLocation es2017/async-functions/export/input.js 2:0-2:38
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: []
	path: UIDPath<es2017/async-functions/export/input.js>
	loc: SourceLocation es2017/async-functions/export/input.js 1:0-3:0
}
```

### `diagnostics`

```

```
