# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > yield > parameter-name-function`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "fn"
				loc: SourceLocation es2015/yield/parameter-name-function/input.js 1:9-1:11 (fn)
			}
			body: JSBlockStatement {
				body: []
				directives: []
				loc: SourceLocation es2015/yield/parameter-name-function/input.js 1:19-1:21
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: [
					JSBindingIdentifier {
						name: "yield"
						meta: JSPatternMeta {
							loc: SourceLocation es2015/yield/parameter-name-function/input.js 1:12-1:17
						}
						loc: SourceLocation es2015/yield/parameter-name-function/input.js 1:12-1:17 (yield)
					}
				]
				loc: SourceLocation es2015/yield/parameter-name-function/input.js 1:11-1:18
			}
			loc: SourceLocation es2015/yield/parameter-name-function/input.js 1:0-1:21
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/yield/parameter-name-function/input.js>
	loc: SourceLocation es2015/yield/parameter-name-function/input.js 1:0-1:21
}
```

### `diagnostics`

```

```
