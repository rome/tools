# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 536`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "const"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "a"
							loc: SourceLocation core/uncategorised/536/input.js 1:6-1:7 (a)
						}
						loc: SourceLocation core/uncategorised/536/input.js 1:6-1:7
					}
				]
				loc: SourceLocation core/uncategorised/536/input.js 1:0-1:8
			}
			loc: SourceLocation core/uncategorised/536/input.js 1:0-1:8
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
				message: RAW_MARKUP {value: "A constant must have an initializer"}
			}
			location: {
				language: "js"
				path: UIDPath<core/uncategorised/536/input.js>
				end: Position 1:7
				start: Position 1:6
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<core/uncategorised/536/input.js>
	loc: SourceLocation core/uncategorised/536/input.js 1:0-1:8
}
```

### `diagnostics`

```

 core/uncategorised/536/input.js:1:6 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ A constant must have an initializer

    const a;
          ^


```
