# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > types > reference-generic-nested`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "let"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "x"
							meta: JSPatternMeta {
								typeAnnotation: TSTypeReference {
									typeName: JSReferenceIdentifier {
										name: "Array"
										loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:7-1:12 (Array)
									}
									typeParameters: TSTypeParameterInstantiation {
										params: [
											TSTypeReference {
												typeName: JSReferenceIdentifier {
													name: "Array"
													loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:13-1:18 (Array)
												}
												typeParameters: TSTypeParameterInstantiation {
													params: [
														TSNumberKeywordTypeAnnotation {
															loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:19-1:25
														}
													]
													loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:18-1:26
												}
												loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:13-1:26
											}
										]
										loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:12-1:27
									}
									loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:7-1:27
								}
								loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:4-1:27
							}
							loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:4-1:27
						}
						loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:4-1:27
					}
				]
				loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:0-1:28
			}
			loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:0-1:28
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/types/reference-generic-nested/input.ts>
	loc: SourceLocation typescript/types/reference-generic-nested/input.ts 1:0-2:0
}
```

### `diagnostics`

```

```
