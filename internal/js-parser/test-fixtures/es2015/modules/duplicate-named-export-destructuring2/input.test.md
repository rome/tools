# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > modules > duplicate-named-export-destructuring2`

### `ast`

```javascript
JSRoot {
	body: [
		JSExportLocalDeclaration {
			exportKind: "value"
			declaration: JSFunctionDeclaration {
				id: JSBindingIdentifier {
					name: "foo"
					loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 1:16-1:19 (foo)
				}
				body: JSBlockStatement {
					body: []
					directives: []
					loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 1:22-1:24
				}
				head: JSFunctionHead {
					async: false
					generator: false
					hasHoistedVars: false
					params: []
					loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 1:19-1:21
				}
				loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 1:7-1:24
			}
			loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 1:0-1:24
		}
		JSEmptyStatement {
			loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 1:24-1:25
		}
		JSExportLocalDeclaration {
			exportKind: "value"
			declaration: JSVariableDeclarationStatement {
				declaration: JSVariableDeclaration {
					kind: "const"
					declarations: [
						JSVariableDeclarator {
							id: JSBindingObjectPattern {
								properties: [
									JSBindingObjectPatternProperty {
										key: JSStaticPropertyKey {
											value: JSIdentifier {
												name: "foo"
												loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 2:15-2:18 (foo)
											}
											loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 2:15-2:18
										}
										value: JSBindingIdentifier {
											name: "foo"
											loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 2:15-2:18 (foo)
										}
										loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 2:15-2:18
									}
								]
								loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 2:13-2:20
							}
							init: JSReferenceIdentifier {
								name: "bar"
								loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 2:23-2:26 (bar)
							}
							loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 2:13-2:26
						}
					]
					loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 2:7-2:27
				}
				loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 2:7-2:27
			}
			loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 2:0-2:27
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: []
	path: UIDPath<es2015/modules/duplicate-named-export-destructuring2/input.js>
	loc: SourceLocation es2015/modules/duplicate-named-export-destructuring2/input.js 1:0-3:0
}
```

### `diagnostics`

```

```
