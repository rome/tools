# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0256`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "t"
				loc: SourceLocation esprima/invalid-syntax/migrated_0256/input.js 1:9-1:10 (t)
			}
			body: JSBlockStatement {
				body: [
					JSEmptyStatement {
						loc: SourceLocation esprima/invalid-syntax/migrated_0256/input.js 1:15-1:16
					}
					JSEmptyStatement {
						loc: SourceLocation esprima/invalid-syntax/migrated_0256/input.js 1:18-1:19
					}
				]
				directives: []
				loc: SourceLocation esprima/invalid-syntax/migrated_0256/input.js 1:13-1:19
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: []
				loc: SourceLocation esprima/invalid-syntax/migrated_0256/input.js 1:10-1:12
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0256/input.js 1:0-1:19
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: [
					log {
						category: "info"
						text: [RAW_MARKUP {value: "We expected to find the closing character <emphasis>"}, "}", RAW_MARKUP {value: "</emphasis> here"}]
					}
					frame {
						location: SourceLocation esprima/invalid-syntax/migrated_0256/input.js 2:0-2:0
					}
				]
				category: ["parse"]
				categoryValue: "js"
				message: [RAW_MARKUP {value: "Unclosed <emphasis>"}, "block", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0256/input.js>
				end: Position 1:13
				start: Position 1:13
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0256/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0256/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0256/input.js:1:13 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unclosed block

    function t() { ;  ;
                 ^

  ℹ We expected to find the closing character } here

    function t() { ;  ;


```
