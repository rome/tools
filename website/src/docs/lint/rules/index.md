---
title: Lint Rules
layout: layouts/page.liquid
layout-type: split
main-class: rules
eleventyNavigation:
  key: lint-rules
  parent: linting
  title: Rules
---

# Rules

<section>
<h2>Correctness</h2>

This group should includes those rules that are meant to prevent possible bugs and misuse of the language.
<div class="rule">
<h3 data-toc-exclude id="noArguments">
	<a href="/docs/lint/rules/noArguments">noArguments (since v0.7.0)</a>
	<a class="header-anchor" href="#noArguments"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow the use of <code>arguments</code>
</div>
<div class="rule">
<h3 data-toc-exclude id="noAsyncPromiseExecutor">
	<a href="/docs/lint/rules/noAsyncPromiseExecutor">noAsyncPromiseExecutor (since v0.7.0)</a>
	<a class="header-anchor" href="#noAsyncPromiseExecutor"></a>
	<span class="recommended">recommended</span>
</h3>
Disallows using an async function as a Promise executor.
</div>
<div class="rule">
<h3 data-toc-exclude id="noCatchAssign">
	<a href="/docs/lint/rules/noCatchAssign">noCatchAssign (since v0.7.0)</a>
	<a class="header-anchor" href="#noCatchAssign"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow reassigning exceptions in catch clauses
</div>
<div class="rule">
<h3 data-toc-exclude id="noCommentText">
	<a href="/docs/lint/rules/noCommentText">noCommentText (since v0.7.0)</a>
	<a class="header-anchor" href="#noCommentText"></a>
	<span class="recommended">recommended</span>
</h3>
Prevent comments from being inserted as text nodes
</div>
<div class="rule">
<h3 data-toc-exclude id="noCompareNegZero">
	<a href="/docs/lint/rules/noCompareNegZero">noCompareNegZero (since v0.7.0)</a>
	<a class="header-anchor" href="#noCompareNegZero"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow comparing against <code>-0</code>
</div>
<div class="rule">
<h3 data-toc-exclude id="noDebugger">
	<a href="/docs/lint/rules/noDebugger">noDebugger (since v0.7.0)</a>
	<a class="header-anchor" href="#noDebugger"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow the use of <code>debugger</code>
</div>
<div class="rule">
<h3 data-toc-exclude id="noDelete">
	<a href="/docs/lint/rules/noDelete">noDelete (since v0.7.0)</a>
	<a class="header-anchor" href="#noDelete"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow the use of the <code>delete</code> operator
</div>
<div class="rule">
<h3 data-toc-exclude id="noDoubleEquals">
	<a href="/docs/lint/rules/noDoubleEquals">noDoubleEquals (since v0.7.0)</a>
	<a class="header-anchor" href="#noDoubleEquals"></a>
	<span class="recommended">recommended</span>
</h3>
Require the use of <code>===</code> and <code>!==</code>
</div>
<div class="rule">
<h3 data-toc-exclude id="noDupeArgs">
	<a href="/docs/lint/rules/noDupeArgs">noDupeArgs (since v0.9.0)</a>
	<a class="header-anchor" href="#noDupeArgs"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow duplicate function arguments name.
</div>
<div class="rule">
<h3 data-toc-exclude id="noEmptyPattern">
	<a href="/docs/lint/rules/noEmptyPattern">noEmptyPattern (since v0.7.0)</a>
	<a class="header-anchor" href="#noEmptyPattern"></a>
	<span class="recommended">recommended</span>
</h3>
Disallows empty destructuring patterns.
</div>
<div class="rule">
<h3 data-toc-exclude id="noExtraBooleanCast">
	<a href="/docs/lint/rules/noExtraBooleanCast">noExtraBooleanCast (since v0.9.0)</a>
	<a class="header-anchor" href="#noExtraBooleanCast"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow unnecessary boolean casts
</div>
<div class="rule">
<h3 data-toc-exclude id="noFunctionAssign">
	<a href="/docs/lint/rules/noFunctionAssign">noFunctionAssign (since v0.7.0)</a>
	<a class="header-anchor" href="#noFunctionAssign"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow reassigning function declarations.
</div>
<div class="rule">
<h3 data-toc-exclude id="noImplicitBoolean">
	<a href="/docs/lint/rules/noImplicitBoolean">noImplicitBoolean (since v0.7.0)</a>
	<a class="header-anchor" href="#noImplicitBoolean"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow implicit <code>true</code> values on JSX boolean attributes
</div>
<div class="rule">
<h3 data-toc-exclude id="noImportAssign">
	<a href="/docs/lint/rules/noImportAssign">noImportAssign (since v0.9.0)</a>
	<a class="header-anchor" href="#noImportAssign"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow assigning to imported bindings
</div>
<div class="rule">
<h3 data-toc-exclude id="noLabelVar">
	<a href="/docs/lint/rules/noLabelVar">noLabelVar (since v0.7.0)</a>
	<a class="header-anchor" href="#noLabelVar"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow labels that share a name with a variable
</div>
<div class="rule">
<h3 data-toc-exclude id="noMultipleSpacesInRegularExpressionLiterals">
	<a href="/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals">noMultipleSpacesInRegularExpressionLiterals (since v0.7.0)</a>
	<a class="header-anchor" href="#noMultipleSpacesInRegularExpressionLiterals"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow unclear usage of multiple space characters in regular expression literals
</div>
<div class="rule">
<h3 data-toc-exclude id="noShadowRestrictedNames">
	<a href="/docs/lint/rules/noShadowRestrictedNames">noShadowRestrictedNames (since v0.9.0)</a>
	<a class="header-anchor" href="#noShadowRestrictedNames"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow identifiers from shadowing restricted names.
</div>
<div class="rule">
<h3 data-toc-exclude id="noSparseArray">
	<a href="/docs/lint/rules/noSparseArray">noSparseArray (since v0.7.0)</a>
	<a class="header-anchor" href="#noSparseArray"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow sparse arrays
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnnecessaryContinue">
	<a href="/docs/lint/rules/noUnnecessaryContinue">noUnnecessaryContinue (since v0.7.0)</a>
	<a class="header-anchor" href="#noUnnecessaryContinue"></a>
	<span class="recommended">recommended</span>
</h3>
Avoid using unnecessary <code>ContinueStatement</code>.
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnsafeNegation">
	<a href="/docs/lint/rules/noUnsafeNegation">noUnsafeNegation (since v0.7.0)</a>
	<a class="header-anchor" href="#noUnsafeNegation"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow using unsafe negation.
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnusedTemplateLiteral">
	<a href="/docs/lint/rules/noUnusedTemplateLiteral">noUnusedTemplateLiteral (since v0.7.0)</a>
	<a class="header-anchor" href="#noUnusedTemplateLiteral"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow template literals if interpolation and special-character handling are not needed
</div>
<div class="rule">
<h3 data-toc-exclude id="useBlockStatements">
	<a href="/docs/lint/rules/useBlockStatements">useBlockStatements (since v0.7.0)</a>
	<a class="header-anchor" href="#useBlockStatements"></a>
	<span class="recommended">recommended</span>
</h3>
Requires following curly brace conventions.
JavaScript allows the omission of curly braces when a block contains only one statement. However, it is considered by many to be best practice to never omit curly braces around blocks, even when they are optional, because it can lead to bugs and reduces code clarity.
</div>
<div class="rule">
<h3 data-toc-exclude id="useOptionalChain">
	<a href="/docs/lint/rules/useOptionalChain">useOptionalChain (since v0.10.0)</a>
	<a class="header-anchor" href="#useOptionalChain"></a>
	<span class="recommended">recommended</span>
</h3>
Enforce using concise optional chain instead of chained logical expressions.
</div>
<div class="rule">
<h3 data-toc-exclude id="useSimplifiedLogicExpression">
	<a href="/docs/lint/rules/useSimplifiedLogicExpression">useSimplifiedLogicExpression (since v0.7.0)</a>
	<a class="header-anchor" href="#useSimplifiedLogicExpression"></a>
	<span class="recommended">recommended</span>
</h3>
Discard redundant terms from logical expressions.
</div>
<div class="rule">
<h3 data-toc-exclude id="useSingleCaseStatement">
	<a href="/docs/lint/rules/useSingleCaseStatement">useSingleCaseStatement (since v0.7.0)</a>
	<a class="header-anchor" href="#useSingleCaseStatement"></a>
	<span class="recommended">recommended</span>
</h3>
Enforces case clauses have a single statement, emits a quick fix wrapping
the statements in a block
</div>
<div class="rule">
<h3 data-toc-exclude id="useSingleVarDeclarator">
	<a href="/docs/lint/rules/useSingleVarDeclarator">useSingleVarDeclarator (since v0.7.0)</a>
	<a class="header-anchor" href="#useSingleVarDeclarator"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow multiple variable declarations in the same variable statement
</div>
<div class="rule">
<h3 data-toc-exclude id="useTemplate">
	<a href="/docs/lint/rules/useTemplate">useTemplate (since v0.7.0)</a>
	<a class="header-anchor" href="#useTemplate"></a>
	<span class="recommended">recommended</span>
</h3>
Template literals are preferred over string concatenation.
</div>
<div class="rule">
<h3 data-toc-exclude id="useValidTypeof">
	<a href="/docs/lint/rules/useValidTypeof">useValidTypeof (since v0.7.0)</a>
	<a class="header-anchor" href="#useValidTypeof"></a>
	<span class="recommended">recommended</span>
</h3>
This rule verifies the result of <code>typeof $expr</code> unary expressions is being
compared to valid values, either string literals containing valid type
names or other <code>typeof</code> expressions
</div>
<div class="rule">
<h3 data-toc-exclude id="useWhile">
	<a href="/docs/lint/rules/useWhile">useWhile (since v0.7.0)</a>
	<a class="header-anchor" href="#useWhile"></a>
	<span class="recommended">recommended</span>
</h3>
Enforce the use of <code>while</code> loops instead of <code>for</code> loops when the
initializer and update expressions are not needed
</div>
</section>
<section>
<h2>Nursery</h2>

Rules that are being written. Rules under this category are meant to be considered unstable or buggy.

Rules can be downgraded to this category in case some path release is needed. After an arbitrary amount of time, the team can decide
to promote these rules into a more appropriate category.
<div class="rule">
<h3 data-toc-exclude id="noDangerouslySetInnerHtml">
	<a href="/docs/lint/rules/noDangerouslySetInnerHtml">noDangerouslySetInnerHtml (since v0.10.0)</a>
	<a class="header-anchor" href="#noDangerouslySetInnerHtml"></a>
</h3>
Prevent the usage of dangerous JSX props
</div>
<div class="rule">
<h3 data-toc-exclude id="noNewSymbol">
	<a href="/docs/lint/rules/noNewSymbol">noNewSymbol (since v0.10.0)</a>
	<a class="header-anchor" href="#noNewSymbol"></a>
</h3>
Disallow <code>new</code> operators with the <code>Symbol</code> object
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnreachable">
	<a href="/docs/lint/rules/noUnreachable">noUnreachable (since v0.7.0)</a>
	<a class="header-anchor" href="#noUnreachable"></a>
</h3>
Disallow unreachable code
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnusedVariables">
	<a href="/docs/lint/rules/noUnusedVariables">noUnusedVariables (since v0.9.0)</a>
	<a class="header-anchor" href="#noUnusedVariables"></a>
</h3>
Disallow unused variables.
</div>
<div class="rule">
<h3 data-toc-exclude id="useCamelCase">
	<a href="/docs/lint/rules/useCamelCase">useCamelCase (since v0.8.0)</a>
	<a class="header-anchor" href="#useCamelCase"></a>
</h3>
Enforce camel case naming convention.
</div>
</section>
<section>
<h2>Style</h2>

Rules that focus mostly on making the code more consistent.
<div class="rule">
<h3 data-toc-exclude id="noNegationElse">
	<a href="/docs/lint/rules/noNegationElse">noNegationElse (since v0.7.0)</a>
	<a class="header-anchor" href="#noNegationElse"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow negation in the condition of an <code>if</code> statement if it has an <code>else</code> clause
</div>
<div class="rule">
<h3 data-toc-exclude id="noShoutyConstants">
	<a href="/docs/lint/rules/noShoutyConstants">noShoutyConstants (since v0.7.0)</a>
	<a class="header-anchor" href="#noShoutyConstants"></a>
	<span class="recommended">recommended</span>
</h3>
Disallow the use of constants which its value is the upper-case version of its name.
</div>
<div class="rule">
<h3 data-toc-exclude id="useSelfClosingElements">
	<a href="/docs/lint/rules/useSelfClosingElements">useSelfClosingElements (since v0.7.0)</a>
	<a class="header-anchor" href="#useSelfClosingElements"></a>
	<span class="recommended">recommended</span>
</h3>
Prevent extra closing tags for components without children
</div>
<div class="rule">
<h3 data-toc-exclude id="useShorthandArrayType">
	<a href="/docs/lint/rules/useShorthandArrayType">useShorthandArrayType (since v0.7.0)</a>
	<a class="header-anchor" href="#useShorthandArrayType"></a>
	<span class="recommended">recommended</span>
</h3>
When expressing array types, this rule promotes the usage of <code>T[]</code> shorthand instead of <code>Array&lt;T&gt;</code>.
</div>
</section>
