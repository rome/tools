# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > let > let-at-binding-list-fail-1`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "let"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingObjectPattern {
							properties: [
								JSBindingObjectPatternProperty {
									key: JSStaticPropertyKey {
										value: JSIdentifier {
											name: "let"
											loc: SourceLocation es2015/let/let-at-binding-list-fail-1/input.js 1:6-1:9 (let)
										}
										loc: SourceLocation es2015/let/let-at-binding-list-fail-1/input.js 1:6-1:9
									}
									value: JSBindingIdentifier {
										name: "let"
										loc: SourceLocation es2015/let/let-at-binding-list-fail-1/input.js 1:6-1:9 (let)
									}
									loc: SourceLocation es2015/let/let-at-binding-list-fail-1/input.js 1:6-1:9
								}
							]
							loc: SourceLocation es2015/let/let-at-binding-list-fail-1/input.js 1:4-1:11
						}
						init: JSObjectExpression {
							properties: []
							loc: SourceLocation es2015/let/let-at-binding-list-fail-1/input.js 1:14-1:16
						}
						loc: SourceLocation es2015/let/let-at-binding-list-fail-1/input.js 1:4-1:16
					}
				]
				loc: SourceLocation es2015/let/let-at-binding-list-fail-1/input.js 1:0-1:17
			}
			loc: SourceLocation es2015/let/let-at-binding-list-fail-1/input.js 1:0-1:17
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/let/let-at-binding-list-fail-1/input.js>
	loc: SourceLocation es2015/let/let-at-binding-list-fail-1/input.js 1:0-2:0
}
```

### `diagnostics`

```

```
