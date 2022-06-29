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
<h2>JavaScript</h2>
<div class="rule">
<h3 data-toc-exclude id="noAsyncPromiseExecutor">
	<a href="/docs/lint/rules/noAsyncPromiseExecutor">noAsyncPromiseExecutor</a>
	<a class="header-anchor" href="#noAsyncPromiseExecutor"></a>
</h3>
Disallows using an async function as a Promise executor.
</div>
<div class="rule">
<h3 data-toc-exclude id="noCompareNegZero">
	<a href="/docs/lint/rules/noCompareNegZero">noCompareNegZero</a>
	<a class="header-anchor" href="#noCompareNegZero"></a>
</h3>
Disallow comparing against <code>-0</code>
</div>
<div class="rule">
<h3 data-toc-exclude id="noDebugger">
	<a href="/docs/lint/rules/noDebugger">noDebugger</a>
	<a class="header-anchor" href="#noDebugger"></a>
</h3>
Disallow the use of <code>debugger</code>
</div>
<div class="rule">
<h3 data-toc-exclude id="noDelete">
	<a href="/docs/lint/rules/noDelete">noDelete</a>
	<a class="header-anchor" href="#noDelete"></a>
</h3>
Disallow the use of the <code>delete</code> operator
</div>
<div class="rule">
<h3 data-toc-exclude id="noDoubleEquals">
	<a href="/docs/lint/rules/noDoubleEquals">noDoubleEquals</a>
	<a class="header-anchor" href="#noDoubleEquals"></a>
</h3>
Require the use of <code>===</code> and <code>!==</code>
</div>
<div class="rule">
<h3 data-toc-exclude id="noEmptyPattern">
	<a href="/docs/lint/rules/noEmptyPattern">noEmptyPattern</a>
	<a class="header-anchor" href="#noEmptyPattern"></a>
</h3>
Disallows empty destructuring patterns.
</div>
<div class="rule">
<h3 data-toc-exclude id="noImplicitBoolean">
	<a href="/docs/lint/rules/noImplicitBoolean">noImplicitBoolean</a>
	<a class="header-anchor" href="#noImplicitBoolean"></a>
</h3>
Disallow implicit <code>true</code> values on JSX boolean attributes
</div>
<div class="rule">
<h3 data-toc-exclude id="noMultipleSpacesInRegularExpressionLiterals">
	<a href="/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals">noMultipleSpacesInRegularExpressionLiterals</a>
	<a class="header-anchor" href="#noMultipleSpacesInRegularExpressionLiterals"></a>
</h3>
Disallow unclear usage of multiple space characters in regular expression literals
</div>
<div class="rule">
<h3 data-toc-exclude id="noNegationElse">
	<a href="/docs/lint/rules/noNegationElse">noNegationElse</a>
	<a class="header-anchor" href="#noNegationElse"></a>
</h3>
Disallow negation in the condition of an <code>if</code> statement if it has an <code>else</code> clause
</div>
<div class="rule">
<h3 data-toc-exclude id="noSparseArray">
	<a href="/docs/lint/rules/noSparseArray">noSparseArray</a>
	<a class="header-anchor" href="#noSparseArray"></a>
</h3>
Disallow sparse arrays
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnnecessaryContinue">
	<a href="/docs/lint/rules/noUnnecessaryContinue">noUnnecessaryContinue</a>
	<a class="header-anchor" href="#noUnnecessaryContinue"></a>
</h3>
Avoid using unnecessary <code>ContinueStatement</code>.
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnsafeNegation">
	<a href="/docs/lint/rules/noUnsafeNegation">noUnsafeNegation</a>
	<a class="header-anchor" href="#noUnsafeNegation"></a>
</h3>
Disallow using unsafe negation.
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnusedTemplateLiteral">
	<a href="/docs/lint/rules/noUnusedTemplateLiteral">noUnusedTemplateLiteral</a>
	<a class="header-anchor" href="#noUnusedTemplateLiteral"></a>
</h3>
Disallow template literals if interpolation and special-character handling are not needed
</div>
<div class="rule">
<h3 data-toc-exclude id="useBlockStatements">
	<a href="/docs/lint/rules/useBlockStatements">useBlockStatements</a>
	<a class="header-anchor" href="#useBlockStatements"></a>
</h3>
Requires following curly brace conventions.
JavaScript allows the omission of curly braces when a block contains only one statement. However, it is considered by many to be best practice to never omit curly braces around blocks, even when they are optional, because it can lead to bugs and reduces code clarity.
</div>
<div class="rule">
<h3 data-toc-exclude id="useSelfClosingElements">
	<a href="/docs/lint/rules/useSelfClosingElements">useSelfClosingElements</a>
	<a class="header-anchor" href="#useSelfClosingElements"></a>
</h3>
Prevent extra closing tags for components without children
</div>
<div class="rule">
<h3 data-toc-exclude id="useShorthandArrayType">
	<a href="/docs/lint/rules/useShorthandArrayType">useShorthandArrayType</a>
	<a class="header-anchor" href="#useShorthandArrayType"></a>
</h3>
When expressing array types, this rule promotes the usage of <code>T[]</code> shorthand instead of <code>Array&lt;T&gt;</code>.
</div>
<div class="rule">
<h3 data-toc-exclude id="useSimplifiedLogicExpression">
	<a href="/docs/lint/rules/useSimplifiedLogicExpression">useSimplifiedLogicExpression</a>
	<a class="header-anchor" href="#useSimplifiedLogicExpression"></a>
</h3>
Discard redundant terms from logical expressions.
</div>
<div class="rule">
<h3 data-toc-exclude id="useSingleCaseStatement">
	<a href="/docs/lint/rules/useSingleCaseStatement">useSingleCaseStatement</a>
	<a class="header-anchor" href="#useSingleCaseStatement"></a>
</h3>
Enforces case clauses have a single statement, emits a quick fix wrapping
the statements in a block
</div>
<div class="rule">
<h3 data-toc-exclude id="useSingleVarDeclarator">
	<a href="/docs/lint/rules/useSingleVarDeclarator">useSingleVarDeclarator</a>
	<a class="header-anchor" href="#useSingleVarDeclarator"></a>
</h3>
Disallow multiple variable declarations in the same variable statement
</div>
<div class="rule">
<h3 data-toc-exclude id="useValidTypeof">
	<a href="/docs/lint/rules/useValidTypeof">useValidTypeof</a>
	<a class="header-anchor" href="#useValidTypeof"></a>
</h3>
This rule verifies the result of <code>typeof $expr</code> unary expressions is being
compared to valid values, either string literals containing valid type
names or other <code>typeof</code> expressions
</div>
<div class="rule">
<h3 data-toc-exclude id="useWhile">
	<a href="/docs/lint/rules/useWhile">useWhile</a>
	<a class="header-anchor" href="#useWhile"></a>
</h3>
Enforce the use of <code>while</code> loops instead of <code>for</code> loops when the
initializer and update expressions are not needed
</div>
</section>
