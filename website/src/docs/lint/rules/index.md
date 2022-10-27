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


## Correctness

Rules that detect incorrect or useless code.
<section class="rule">
<h3 data-toc-exclude id="noArguments">
	<a href="/docs/lint/rules/noArguments">noArguments</a>
	<span class="recommended">recommended</span>
</h3>
Disallow the use of <code>arguments</code>
</section>
<section class="rule">
<h3 data-toc-exclude id="noAsyncPromiseExecutor">
	<a href="/docs/lint/rules/noAsyncPromiseExecutor">noAsyncPromiseExecutor</a>
	<span class="recommended">recommended</span>
</h3>
Disallows using an async function as a Promise executor.
</section>
<section class="rule">
<h3 data-toc-exclude id="noCatchAssign">
	<a href="/docs/lint/rules/noCatchAssign">noCatchAssign</a>
	<span class="recommended">recommended</span>
</h3>
Disallow reassigning exceptions in catch clauses
</section>
<section class="rule">
<h3 data-toc-exclude id="noCommentText">
	<a href="/docs/lint/rules/noCommentText">noCommentText</a>
	<span class="recommended">recommended</span>
</h3>
Prevent comments from being inserted as text nodes
</section>
<section class="rule">
<h3 data-toc-exclude id="noCompareNegZero">
	<a href="/docs/lint/rules/noCompareNegZero">noCompareNegZero</a>
	<span class="recommended">recommended</span>
</h3>
Disallow comparing against <code>-0</code>
</section>
<section class="rule">
<h3 data-toc-exclude id="noDebugger">
	<a href="/docs/lint/rules/noDebugger">noDebugger</a>
	<span class="recommended">recommended</span>
</h3>
Disallow the use of <code>debugger</code>
</section>
<section class="rule">
<h3 data-toc-exclude id="noDelete">
	<a href="/docs/lint/rules/noDelete">noDelete</a>
	<span class="recommended">recommended</span>
</h3>
Disallow the use of the <code>delete</code> operator
</section>
<section class="rule">
<h3 data-toc-exclude id="noDoubleEquals">
	<a href="/docs/lint/rules/noDoubleEquals">noDoubleEquals</a>
	<span class="recommended">recommended</span>
</h3>
Require the use of <code>===</code> and <code>!==</code>
</section>
<section class="rule">
<h3 data-toc-exclude id="noDupeArgs">
	<a href="/docs/lint/rules/noDupeArgs">noDupeArgs</a>
	<span class="recommended">recommended</span>
</h3>
Disallow duplicate function arguments name.
</section>
<section class="rule">
<h3 data-toc-exclude id="noEmptyPattern">
	<a href="/docs/lint/rules/noEmptyPattern">noEmptyPattern</a>
	<span class="recommended">recommended</span>
</h3>
Disallows empty destructuring patterns.
</section>
<section class="rule">
<h3 data-toc-exclude id="noExtraBooleanCast">
	<a href="/docs/lint/rules/noExtraBooleanCast">noExtraBooleanCast</a>
	<span class="recommended">recommended</span>
</h3>
Disallow unnecessary boolean casts
</section>
<section class="rule">
<h3 data-toc-exclude id="noFunctionAssign">
	<a href="/docs/lint/rules/noFunctionAssign">noFunctionAssign</a>
	<span class="recommended">recommended</span>
</h3>
Disallow reassigning function declarations.
</section>
<section class="rule">
<h3 data-toc-exclude id="noImplicitBoolean">
	<a href="/docs/lint/rules/noImplicitBoolean">noImplicitBoolean</a>
	<span class="recommended">recommended</span>
</h3>
Disallow implicit <code>true</code> values on JSX boolean attributes
</section>
<section class="rule">
<h3 data-toc-exclude id="noImportAssign">
	<a href="/docs/lint/rules/noImportAssign">noImportAssign</a>
	<span class="recommended">recommended</span>
</h3>
Disallow assigning to imported bindings
</section>
<section class="rule">
<h3 data-toc-exclude id="noLabelVar">
	<a href="/docs/lint/rules/noLabelVar">noLabelVar</a>
	<span class="recommended">recommended</span>
</h3>
Disallow labels that share a name with a variable
</section>
<section class="rule">
<h3 data-toc-exclude id="noMultipleSpacesInRegularExpressionLiterals">
	<a href="/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals">noMultipleSpacesInRegularExpressionLiterals</a>
	<span class="recommended">recommended</span>
</h3>
Disallow unclear usage of multiple space characters in regular expression literals
</section>
<section class="rule">
<h3 data-toc-exclude id="noShadowRestrictedNames">
	<a href="/docs/lint/rules/noShadowRestrictedNames">noShadowRestrictedNames</a>
	<span class="recommended">recommended</span>
</h3>
Disallow identifiers from shadowing restricted names.
</section>
<section class="rule">
<h3 data-toc-exclude id="noSparseArray">
	<a href="/docs/lint/rules/noSparseArray">noSparseArray</a>
	<span class="recommended">recommended</span>
</h3>
Disallow sparse arrays
</section>
<section class="rule">
<h3 data-toc-exclude id="noUnnecessaryContinue">
	<a href="/docs/lint/rules/noUnnecessaryContinue">noUnnecessaryContinue</a>
	<span class="recommended">recommended</span>
</h3>
Avoid using unnecessary <code>ContinueStatement</code>.
</section>
<section class="rule">
<h3 data-toc-exclude id="noUnsafeNegation">
	<a href="/docs/lint/rules/noUnsafeNegation">noUnsafeNegation</a>
	<span class="recommended">recommended</span>
</h3>
Disallow using unsafe negation.
</section>
<section class="rule">
<h3 data-toc-exclude id="noUnusedTemplateLiteral">
	<a href="/docs/lint/rules/noUnusedTemplateLiteral">noUnusedTemplateLiteral</a>
	<span class="recommended">recommended</span>
</h3>
Disallow template literals if interpolation and special-character handling are not needed
</section>
<section class="rule">
<h3 data-toc-exclude id="useBlockStatements">
	<a href="/docs/lint/rules/useBlockStatements">useBlockStatements</a>
	<span class="recommended">recommended</span>
</h3>
Requires following curly brace conventions.
JavaScript allows the omission of curly braces when a block contains only one statement. However, it is considered by many to be best practice to never omit curly braces around blocks, even when they are optional, because it can lead to bugs and reduces code clarity.
</section>
<section class="rule">
<h3 data-toc-exclude id="useSimplifiedLogicExpression">
	<a href="/docs/lint/rules/useSimplifiedLogicExpression">useSimplifiedLogicExpression</a>
	<span class="recommended">recommended</span>
</h3>
Discard redundant terms from logical expressions.
</section>
<section class="rule">
<h3 data-toc-exclude id="useSingleCaseStatement">
	<a href="/docs/lint/rules/useSingleCaseStatement">useSingleCaseStatement</a>
	<span class="recommended">recommended</span>
</h3>
Enforces case clauses have a single statement, emits a quick fix wrapping
the statements in a block
</section>
<section class="rule">
<h3 data-toc-exclude id="useSingleVarDeclarator">
	<a href="/docs/lint/rules/useSingleVarDeclarator">useSingleVarDeclarator</a>
	<span class="recommended">recommended</span>
</h3>
Disallow multiple variable declarations in the same variable statement
</section>
<section class="rule">
<h3 data-toc-exclude id="useTemplate">
	<a href="/docs/lint/rules/useTemplate">useTemplate</a>
	<span class="recommended">recommended</span>
</h3>
Template literals are preferred over string concatenation.
</section>
<section class="rule">
<h3 data-toc-exclude id="useValidTypeof">
	<a href="/docs/lint/rules/useValidTypeof">useValidTypeof</a>
	<span class="recommended">recommended</span>
</h3>
This rule verifies the result of <code>typeof $expr</code> unary expressions is being
compared to valid values, either string literals containing valid type
names or other <code>typeof</code> expressions
</section>
<section class="rule">
<h3 data-toc-exclude id="useWhile">
	<a href="/docs/lint/rules/useWhile">useWhile</a>
	<span class="recommended">recommended</span>
</h3>
Enforce the use of <code>while</code> loops instead of <code>for</code> loops when the
initializer and update expressions are not needed
</section>

## Style

Rules enforcing a consistent way of writing your code. 
<section class="rule">
<h3 data-toc-exclude id="noNegationElse">
	<a href="/docs/lint/rules/noNegationElse">noNegationElse</a>
	<span class="recommended">recommended</span>
</h3>
Disallow negation in the condition of an <code>if</code> statement if it has an <code>else</code> clause
</section>
<section class="rule">
<h3 data-toc-exclude id="noShoutyConstants">
	<a href="/docs/lint/rules/noShoutyConstants">noShoutyConstants</a>
	<span class="recommended">recommended</span>
</h3>
Disallow the use of constants which its value is the upper-case version of its name.
</section>
<section class="rule">
<h3 data-toc-exclude id="useSelfClosingElements">
	<a href="/docs/lint/rules/useSelfClosingElements">useSelfClosingElements</a>
	<span class="recommended">recommended</span>
</h3>
Prevent extra closing tags for components without children
</section>
<section class="rule">
<h3 data-toc-exclude id="useShorthandArrayType">
	<a href="/docs/lint/rules/useShorthandArrayType">useShorthandArrayType</a>
	<span class="recommended">recommended</span>
</h3>
When expressing array types, this rule promotes the usage of <code>T[]</code> shorthand instead of <code>Array&lt;T&gt;</code>.
</section>

## Nursery

New rules that are still under development.

Nursery rules require explicit opt-in via configuration because they may still have bugs or performance problems.
Nursery rules get promoted to other groups once they become stable or may be removed.
<section class="rule">
<h3 data-toc-exclude id="noArrayIndexKey">
	<a href="/docs/lint/rules/noArrayIndexKey">noArrayIndexKey</a>
</h3>
Discourage the usage of Array index in keys.
</section>
<section class="rule">
<h3 data-toc-exclude id="noAutofocus">
	<a href="/docs/lint/rules/noAutofocus">noAutofocus</a>
</h3>
Avoid the <code>autoFocus</code> attribute
</section>
<section class="rule">
<h3 data-toc-exclude id="noBannedTypes">
	<a href="/docs/lint/rules/noBannedTypes">noBannedTypes</a>
</h3>
Disallow certain types.
</section>
<section class="rule">
<h3 data-toc-exclude id="noChildrenProp">
	<a href="/docs/lint/rules/noChildrenProp">noChildrenProp</a>
</h3>
Prevent passing of <strong>children</strong> as props.
</section>
<section class="rule">
<h3 data-toc-exclude id="noConstAssign">
	<a href="/docs/lint/rules/noConstAssign">noConstAssign</a>
</h3>
Prevents from having <code>const</code> variables being re-assigned.
</section>
<section class="rule">
<h3 data-toc-exclude id="noDangerouslySetInnerHtml">
	<a href="/docs/lint/rules/noDangerouslySetInnerHtml">noDangerouslySetInnerHtml</a>
</h3>
Prevent the usage of dangerous JSX props
</section>
<section class="rule">
<h3 data-toc-exclude id="noDangerouslySetInnerHtmlWithChildren">
	<a href="/docs/lint/rules/noDangerouslySetInnerHtmlWithChildren">noDangerouslySetInnerHtmlWithChildren</a>
</h3>
Report when a DOM element or a component uses both <code>children</code> and <code>dangerouslySetInnerHTML</code> prop.
</section>
<section class="rule">
<h3 data-toc-exclude id="noExplicitAny">
	<a href="/docs/lint/rules/noExplicitAny">noExplicitAny</a>
</h3>
Disallow the <code>any</code> type usage
</section>
<section class="rule">
<h3 data-toc-exclude id="noInvalidConstructorSuper">
	<a href="/docs/lint/rules/noInvalidConstructorSuper">noInvalidConstructorSuper</a>
</h3>
Prevents the incorrect use of <code>super()</code> inside classes.
It also checks whether a call <code>super()</code> is missing from classes that extends other constructors.
</section>
<section class="rule">
<h3 data-toc-exclude id="noNewSymbol">
	<a href="/docs/lint/rules/noNewSymbol">noNewSymbol</a>
</h3>
Disallow <code>new</code> operators with the <code>Symbol</code> object
</section>
<section class="rule">
<h3 data-toc-exclude id="noPositiveTabindex">
	<a href="/docs/lint/rules/noPositiveTabindex">noPositiveTabindex</a>
</h3>
Prevent the usage of positive integers on <code>tabIndex</code> property
</section>
<section class="rule">
<h3 data-toc-exclude id="noRenderReturnValue">
	<a href="/docs/lint/rules/noRenderReturnValue">noRenderReturnValue</a>
</h3>
Prevent the usage of the return value of <code>React.render</code>.
</section>
<section class="rule">
<h3 data-toc-exclude id="noRestrictedGlobals">
	<a href="/docs/lint/rules/noRestrictedGlobals">noRestrictedGlobals</a>
</h3>
This rule allows you to specify global variable names that you don’t want to use in your application.
</section>
<section class="rule">
<h3 data-toc-exclude id="noUndeclaredVariables">
	<a href="/docs/lint/rules/noUndeclaredVariables">noUndeclaredVariables</a>
</h3>
Prevents the usage of variables that haven't been declared inside the document
</section>
<section class="rule">
<h3 data-toc-exclude id="noUnreachable">
	<a href="/docs/lint/rules/noUnreachable">noUnreachable</a>
</h3>
Disallow unreachable code
</section>
<section class="rule">
<h3 data-toc-exclude id="noUnusedVariables">
	<a href="/docs/lint/rules/noUnusedVariables">noUnusedVariables</a>
</h3>
Disallow unused variables.
</section>
<section class="rule">
<h3 data-toc-exclude id="noUselessFragments">
	<a href="/docs/lint/rules/noUselessFragments">noUselessFragments</a>
</h3>
Disallow unnecessary fragments
</section>
<section class="rule">
<h3 data-toc-exclude id="noVoidElementsWithChildren">
	<a href="/docs/lint/rules/noVoidElementsWithChildren">noVoidElementsWithChildren</a>
</h3>
This rules prevents void elements (AKA self-closing elements) from having children.
</section>
<section class="rule">
<h3 data-toc-exclude id="useAltText">
	<a href="/docs/lint/rules/useAltText">useAltText</a>
</h3>
It asserts that alternative text to images or areas, help to rely on to screen readers to understand the purpose and the context of the image.
</section>
<section class="rule">
<h3 data-toc-exclude id="useAnchorContent">
	<a href="/docs/lint/rules/useAnchorContent">useAnchorContent</a>
</h3>
Enforce that anchor elements have content and that the content is accessible to screen readers.
</section>
<section class="rule">
<h3 data-toc-exclude id="useBlankTarget">
	<a href="/docs/lint/rules/useBlankTarget">useBlankTarget</a>
</h3>
Disallow <code>target=&quot;_blank&quot;</code> attribute without <code>rel=&quot;noreferrer&quot;</code>
</section>
<section class="rule">
<h3 data-toc-exclude id="useButtonType">
	<a href="/docs/lint/rules/useButtonType">useButtonType</a>
</h3>
Enforces the usage of the attribute <code>type</code> for the element <code>button</code>
</section>
<section class="rule">
<h3 data-toc-exclude id="useCamelCase">
	<a href="/docs/lint/rules/useCamelCase">useCamelCase</a>
</h3>
Enforce camel case naming convention.
</section>
<section class="rule">
<h3 data-toc-exclude id="useExhaustiveDependencies">
	<a href="/docs/lint/rules/useExhaustiveDependencies">useExhaustiveDependencies</a>
</h3>
Enforce all dependencies are correctly specified.
</section>
<section class="rule">
<h3 data-toc-exclude id="useFlatMap">
	<a href="/docs/lint/rules/useFlatMap">useFlatMap</a>
</h3>
Promotes the use of <code>.flatMap()</code> when <code>map().flat()</code> are used together.
</section>
<section class="rule">
<h3 data-toc-exclude id="useFragmentSyntax">
	<a href="/docs/lint/rules/useFragmentSyntax">useFragmentSyntax</a>
</h3>
This rule enforces the use of <code>&lt;&gt;...&lt;/&gt;</code> over <code>&lt;Fragment&gt;...&lt;/Fragment&gt;</code>.
</section>
<section class="rule">
<h3 data-toc-exclude id="useKeyWithClickEvents">
	<a href="/docs/lint/rules/useKeyWithClickEvents">useKeyWithClickEvents</a>
</h3>
Enforce to have the <code>onClick</code> mouse event with the <code>onKeyUp</code>, the <code>onKeyDown</code>, or the <code>noKeyPress</code> keyboard event.
</section>
<section class="rule">
<h3 data-toc-exclude id="useKeyWithMouseEvents">
	<a href="/docs/lint/rules/useKeyWithMouseEvents">useKeyWithMouseEvents</a>
</h3>
Enforce that <code>onMouseOver</code>/<code>onMouseOut</code> are accompanied by <code>onFocus</code>/<code>onBlur</code> for keyboard-only users.
It is important to take into account users with physical disabilities who cannot use a mouse,
who use assistive technology or screenreader.
</section>
<section class="rule">
<h3 data-toc-exclude id="useOptionalChain">
	<a href="/docs/lint/rules/useOptionalChain">useOptionalChain</a>
</h3>
Enforce using concise optional chain instead of chained logical expressions.
</section>
<section class="rule">
<h3 data-toc-exclude id="useValidAnchor">
	<a href="/docs/lint/rules/useValidAnchor">useValidAnchor</a>
</h3>
Enforce that all anchors are valid, and they are navigable elements.
</section>
<section class="rule">
<h3 data-toc-exclude id="useValidForDirection">
	<a href="/docs/lint/rules/useValidForDirection">useValidForDirection</a>
</h3>
Enforce &quot;for&quot; loop update clause moving the counter in the right direction.
</section>
