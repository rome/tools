# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2016 > exponentiation-operator > 14`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentIdentifier {
					name: "INVALID_PLACEHOLDER"
					loc: SourceLocation es2016/exponentiation-operator/14/input.js 1:4-1:4
				}
				right: JSNumericLiteral {
					value: 1
					loc: SourceLocation es2016/exponentiation-operator/14/input.js 1:6-1:7
				}
				loc: SourceLocation es2016/exponentiation-operator/14/input.js 1:0-1:7
			}
			loc: SourceLocation es2016/exponentiation-operator/14/input.js 1:0-1:8
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: [RAW_MARKUP {value: "Unknown start to an "}, "statement expression"]}
			location: {
				language: "js"
				path: UIDPath<es2016/exponentiation-operator/14/input.js>
				end: Position 1:3
				start: Position 1:3
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2016/exponentiation-operator/14/input.js>
	loc: SourceLocation es2016/exponentiation-operator/14/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2016/exponentiation-operator/14/input.js:1:3 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unknown start to an statement expression

    a %*= 1;
       ^


```
