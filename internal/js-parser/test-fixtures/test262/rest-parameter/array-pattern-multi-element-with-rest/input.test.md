# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `test262 > rest-parameter > array-pattern-multi-element-with-rest`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "multiElementWithRest"
				loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:9-1:29 (multiElementWithRest)
			}
			body: JSBlockStatement {
				body: []
				directives: []
				loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:47-1:49
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: []
				rest: JSBindingArrayPattern {
					elements: [
						JSBindingIdentifier {
							name: "a"
							meta: JSPatternMeta {
								loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:34-1:35
							}
							loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:34-1:35 (a)
						}
						JSBindingIdentifier {
							name: "b"
							meta: JSPatternMeta {
								loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:37-1:38
							}
							loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:37-1:38 (b)
						}
					]
					meta: JSPatternMeta {
						loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:33-1:45
					}
					rest: JSBindingIdentifier {
						name: "c"
						meta: JSPatternMeta {
							loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:43-1:44
						}
						loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:43-1:44 (c)
					}
					loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:33-1:45
				}
				loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:29-1:46
			}
			loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:0-1:49
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<test262/rest-parameter/array-pattern-multi-element-with-rest/input.js>
	loc: SourceLocation test262/rest-parameter/array-pattern-multi-element-with-rest/input.js 1:0-2:0
}
```

### `diagnostics`

```

```
