# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-export-declaration > invalid-export-batch-missing-from-clause`

### `ast`

```javascript
JSRoot {
	body: [
		JSExportAllDeclaration {
			source: JSStringLiteral {
				value: ""
				loc: SourceLocation esprima/es2015-export-declaration/invalid-export-batch-missing-from-clause/input.js 2:0-1:8
			}
			loc: SourceLocation esprima/es2015-export-declaration/invalid-export-batch-missing-from-clause/input.js 1:0-1:8
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
				message: RAW_MARKUP {value: "Expected `from` for an export node"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/es2015-export-declaration/invalid-export-batch-missing-from-clause/input.js>
				end: Position 1:8
				start: Position 2:0
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: []
	path: UIDPath<esprima/es2015-export-declaration/invalid-export-batch-missing-from-clause/input.js>
	loc: SourceLocation esprima/es2015-export-declaration/invalid-export-batch-missing-from-clause/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/es2015-export-declaration/invalid-export-batch-missing-from-clause/input.js:2 parse(js) ━━━

  ✖ Expected `from` for an export node

    export *


```
