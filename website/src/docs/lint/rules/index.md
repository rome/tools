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

<!-- GENERATED:START(hash:ae42156f2940a763067781eed8f7d571e45c9179,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->
<section>
<h2>JavaScript</h2>
<p>Rule semantics and descriptions taken from <a href="https://eslint.org/">ESLint</a>. See individual rule docs for direct references.</p>
<div class="rule">
<h3 data-toc-exclude id="noArguments">
	<a href="/docs/lint/rules/js/noArguments">noArguments</a>
	<a class="header-anchor" href="#noArguments"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="noAsyncPromiseExecutor">
	<a href="/docs/lint/rules/js/noAsyncPromiseExecutor">noAsyncPromiseExecutor</a>
	<a class="header-anchor" href="#noAsyncPromiseExecutor"></a>
</h3>
disallow using an async function as a Promise executor
</div>
<div class="rule">
<h3 data-toc-exclude id="noCatchAssign">
	<a href="/docs/lint/rules/js/noCatchAssign">noCatchAssign</a>
	<a class="header-anchor" href="#noCatchAssign"></a>
</h3>
disallow reassigning exceptions in `catch` clauses
</div>
<div class="rule">
<h3 data-toc-exclude id="noCommaOperator">
	<a href="/docs/lint/rules/js/noCommaOperator">noCommaOperator</a>
	<a class="header-anchor" href="#noCommaOperator"></a>
</h3>
disallow comma operators
</div>
<div class="rule">
<h3 data-toc-exclude id="noCompareNegZero">
	<a href="/docs/lint/rules/js/noCompareNegZero">noCompareNegZero</a>
	<a class="header-anchor" href="#noCompareNegZero"></a>
</h3>
disallow comparing against `-0`
</div>
<div class="rule">
<h3 data-toc-exclude id="noCondAssign">
	<a href="/docs/lint/rules/js/noCondAssign">noCondAssign</a>
	<a class="header-anchor" href="#noCondAssign"></a>
</h3>
disallow assignment operators in conditional expressions
</div>
<div class="rule">
<h3 data-toc-exclude id="noDebugger">
	<a href="/docs/lint/rules/js/noDebugger">noDebugger</a>
	<a class="header-anchor" href="#noDebugger"></a>
</h3>
disallow the use of `debugger`
</div>
<div class="rule">
<h3 data-toc-exclude id="noDelete">
	<a href="/docs/lint/rules/js/noDelete">noDelete</a>
	<a class="header-anchor" href="#noDelete"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="noDeleteVars">
	<a href="/docs/lint/rules/js/noDeleteVars">noDeleteVars</a>
	<a class="header-anchor" href="#noDeleteVars"></a>
</h3>
disallow deleting variables
</div>
<div class="rule">
<h3 data-toc-exclude id="noDoubleEquals">
	<a href="/docs/lint/rules/js/noDoubleEquals">noDoubleEquals</a>
	<a class="header-anchor" href="#noDoubleEquals"></a>
</h3>
require the use of `===` and `!==`
</div>
<div class="rule">
<h3 data-toc-exclude id="noDupeArgs">
	<a href="/docs/lint/rules/js/noDupeArgs">noDupeArgs</a>
	<a class="header-anchor" href="#noDupeArgs"></a>
</h3>
disallow duplicate arguments in `function` definitions
</div>
<div class="rule">
<h3 data-toc-exclude id="noDuplicateCase">
	<a href="/docs/lint/rules/js/noDuplicateCase">noDuplicateCase</a>
	<a class="header-anchor" href="#noDuplicateCase"></a>
</h3>
disallow duplicate case labels
</div>
<div class="rule">
<h3 data-toc-exclude id="noDuplicateImportSource">
	<a href="/docs/lint/rules/js/noDuplicateImportSource">noDuplicateImportSource</a>
	<a class="header-anchor" href="#noDuplicateImportSource"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="noDuplicateKeys">
	<a href="/docs/lint/rules/js/noDuplicateKeys">noDuplicateKeys</a>
	<a class="header-anchor" href="#noDuplicateKeys"></a>
</h3>
disallow duplicate keys in object literals
</div>
<div class="rule">
<h3 data-toc-exclude id="noEmptyBlocks">
	<a href="/docs/lint/rules/js/noEmptyBlocks">noEmptyBlocks</a>
	<a class="header-anchor" href="#noEmptyBlocks"></a>
</h3>
disallow empty block statements
</div>
<div class="rule">
<h3 data-toc-exclude id="noExtraBooleanCast">
	<a href="/docs/lint/rules/js/noExtraBooleanCast">noExtraBooleanCast</a>
	<a class="header-anchor" href="#noExtraBooleanCast"></a>
</h3>
disallow unnecessary boolean casts
</div>
<div class="rule">
<h3 data-toc-exclude id="noFunctionAssign">
	<a href="/docs/lint/rules/js/noFunctionAssign">noFunctionAssign</a>
	<a class="header-anchor" href="#noFunctionAssign"></a>
</h3>
disallow reassigning `function` declarations
</div>
<div class="rule">
<h3 data-toc-exclude id="noGetterReturn">
	<a href="/docs/lint/rules/js/noGetterReturn">noGetterReturn</a>
	<a class="header-anchor" href="#noGetterReturn"></a>
</h3>
enforce `return` statements in getters
</div>
<div class="rule">
<h3 data-toc-exclude id="noImportAssign">
	<a href="/docs/lint/rules/js/noImportAssign">noImportAssign</a>
	<a class="header-anchor" href="#noImportAssign"></a>
</h3>
disallow assigning to imported bindings
</div>
<div class="rule">
<h3 data-toc-exclude id="noLabelVar">
	<a href="/docs/lint/rules/js/noLabelVar">noLabelVar</a>
	<a class="header-anchor" href="#noLabelVar"></a>
</h3>
disallow labels that share a name with a variable
</div>
<div class="rule">
<h3 data-toc-exclude id="noNegationElse">
	<a href="/docs/lint/rules/js/noNegationElse">noNegationElse</a>
	<a class="header-anchor" href="#noNegationElse"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="noNestedTernary">
	<a href="/docs/lint/rules/js/noNestedTernary">noNestedTernary</a>
	<a class="header-anchor" href="#noNestedTernary"></a>
</h3>
disallow nested ternary expressions
</div>
<div class="rule">
<h3 data-toc-exclude id="noRestrictedGlobals">
	<a href="/docs/lint/rules/js/noRestrictedGlobals">noRestrictedGlobals</a>
	<a class="header-anchor" href="#noRestrictedGlobals"></a>
</h3>
disallow certain global variables
</div>
<div class="rule">
<h3 data-toc-exclude id="noSetterReturn">
	<a href="/docs/lint/rules/js/noSetterReturn">noSetterReturn</a>
	<a class="header-anchor" href="#noSetterReturn"></a>
</h3>
disallow returning values from setters
</div>
<div class="rule">
<h3 data-toc-exclude id="noShadowRestrictedNames">
	<a href="/docs/lint/rules/js/noShadowRestrictedNames">noShadowRestrictedNames</a>
	<a class="header-anchor" href="#noShadowRestrictedNames"></a>
</h3>
disallow identifiers from shadowing restricted names
</div>
<div class="rule">
<h3 data-toc-exclude id="noShorthandArrayType">
	<a href="/docs/lint/rules/js/noShorthandArrayType">noShorthandArrayType</a>
	<a class="header-anchor" href="#noShorthandArrayType"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="noShoutyConstants">
	<a href="/docs/lint/rules/js/noShoutyConstants">noShoutyConstants</a>
	<a class="header-anchor" href="#noShoutyConstants"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="noSparseArray">
	<a href="/docs/lint/rules/js/noSparseArray">noSparseArray</a>
	<a class="header-anchor" href="#noSparseArray"></a>
</h3>
disallow sparse arrays
</div>
<div class="rule">
<h3 data-toc-exclude id="noTemplateCurlyInString">
	<a href="/docs/lint/rules/js/noTemplateCurlyInString">noTemplateCurlyInString</a>
	<a class="header-anchor" href="#noTemplateCurlyInString"></a>
</h3>
disallow template literal placeholder syntax in regular strings
</div>
<div class="rule">
<h3 data-toc-exclude id="noUndeclaredVariables">
	<a href="/docs/lint/rules/js/noUndeclaredVariables">noUndeclaredVariables</a>
	<a class="header-anchor" href="#noUndeclaredVariables"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnsafeFinally">
	<a href="/docs/lint/rules/js/noUnsafeFinally">noUnsafeFinally</a>
	<a class="header-anchor" href="#noUnsafeFinally"></a>
</h3>
disallow control flow statements in `finally` blocks
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnsafeNegation">
	<a href="/docs/lint/rules/js/noUnsafeNegation">noUnsafeNegation</a>
	<a class="header-anchor" href="#noUnsafeNegation"></a>
</h3>
disallow negating the left operand of relational operators
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnusedTemplateLiteral">
	<a href="/docs/lint/rules/js/noUnusedTemplateLiteral">noUnusedTemplateLiteral</a>
	<a class="header-anchor" href="#noUnusedTemplateLiteral"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnusedVariables">
	<a href="/docs/lint/rules/js/noUnusedVariables">noUnusedVariables</a>
	<a class="header-anchor" href="#noUnusedVariables"></a>
</h3>
disallow unused variables
</div>
<div class="rule">
<h3 data-toc-exclude id="noVar">
	<a href="/docs/lint/rules/js/noVar">noVar</a>
	<a class="header-anchor" href="#noVar"></a>
</h3>
require `let` or `const` instead of `var`
</div>
<div class="rule">
<h3 data-toc-exclude id="useBlockStatements">
	<a href="/docs/lint/rules/js/useBlockStatements">useBlockStatements</a>
	<a class="header-anchor" href="#useBlockStatements"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="useCamelCase">
	<a href="/docs/lint/rules/js/useCamelCase">useCamelCase</a>
	<a class="header-anchor" href="#useCamelCase"></a>
</h3>
enforce camelcase naming convention
</div>
<div class="rule">
<h3 data-toc-exclude id="useDefaultExportBasename">
	<a href="/docs/lint/rules/js/useDefaultExportBasename">useDefaultExportBasename</a>
	<a class="header-anchor" href="#useDefaultExportBasename"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="useDefaultImportBasename">
	<a href="/docs/lint/rules/js/useDefaultImportBasename">useDefaultImportBasename</a>
	<a class="header-anchor" href="#useDefaultImportBasename"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="useFunctionDeclarations">
	<a href="/docs/lint/rules/js/useFunctionDeclarations">useFunctionDeclarations</a>
	<a class="header-anchor" href="#useFunctionDeclarations"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="useSingleCaseStatement">
	<a href="/docs/lint/rules/js/useSingleCaseStatement">useSingleCaseStatement</a>
	<a class="header-anchor" href="#useSingleCaseStatement"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="useSingleVarDeclarator">
	<a href="/docs/lint/rules/js/useSingleVarDeclarator">useSingleVarDeclarator</a>
	<a class="header-anchor" href="#useSingleVarDeclarator"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="useSortedSpecifiers">
	<a href="/docs/lint/rules/js/useSortedSpecifiers">useSortedSpecifiers</a>
	<a class="header-anchor" href="#useSortedSpecifiers"></a>
</h3>
MISSING DOCUMENTATION
</div>
<div class="rule">
<h3 data-toc-exclude id="useTemplate">
	<a href="/docs/lint/rules/js/useTemplate">useTemplate</a>
	<a class="header-anchor" href="#useTemplate"></a>
</h3>
require template literals instead of string concatenation
</div>
<div class="rule">
<h3 data-toc-exclude id="useWhile">
	<a href="/docs/lint/rules/js/useWhile">useWhile</a>
	<a class="header-anchor" href="#useWhile"></a>
</h3>
MISSING DOCUMENTATION
</div>
</section>
<section>
<h2>TypeScript</h2>
<div class="rule">
<h3 data-toc-exclude id="noExplicitAny">
	<a href="/docs/lint/rules/ts/noExplicitAny">noExplicitAny</a>
	<a class="header-anchor" href="#noExplicitAny"></a>
</h3>
it bans the use of `any`
</div>
<div class="rule">
<h3 data-toc-exclude id="useInterfaces">
	<a href="/docs/lint/rules/ts/useInterfaces">useInterfaces</a>
	<a class="header-anchor" href="#useInterfaces"></a>
</h3>
MISSING DOCUMENTATION
</div>
</section>
<section>
<h2>JSX Accessibility</h2>
<p>Rule semantics and descriptions taken from <a href="https://github.com/jsx-eslint/eslint-plugin-jsx-a11y">eslint-plugin-jsx-a11y</a>. See individual rule docs for direct references.</p>
<div class="rule">
<h3 data-toc-exclude id="noAccessKey">
	<a href="/docs/lint/rules/jsx-a11y/noAccessKey">noAccessKey</a>
	<a class="header-anchor" href="#noAccessKey"></a>
</h3>
enforce that the `accessKey` prop is not used on any element to avoid complications with keyboard commands used by a screenreader
</div>
<div class="rule">
<h3 data-toc-exclude id="noAriaUnsupportedElements">
	<a href="/docs/lint/rules/jsx-a11y/noAriaUnsupportedElements">noAriaUnsupportedElements</a>
	<a class="header-anchor" href="#noAriaUnsupportedElements"></a>
</h3>
enforce that elements that do not support ARIA roles, states, and properties do not have those attributes
</div>
<div class="rule">
<h3 data-toc-exclude id="noAutofocus">
	<a href="/docs/lint/rules/jsx-a11y/noAutofocus">noAutofocus</a>
	<a class="header-anchor" href="#noAutofocus"></a>
</h3>
discourage the usage of `autoFocus`
</div>
<div class="rule">
<h3 data-toc-exclude id="noDistractingElements">
	<a href="/docs/lint/rules/jsx-a11y/noDistractingElements">noDistractingElements</a>
	<a class="header-anchor" href="#noDistractingElements"></a>
</h3>
enforce distracting elements are not used
</div>
<div class="rule">
<h3 data-toc-exclude id="noHeaderScope">
	<a href="/docs/lint/rules/jsx-a11y/noHeaderScope">noHeaderScope</a>
	<a class="header-anchor" href="#noHeaderScope"></a>
</h3>
enforce scope prop is only used on `th` elements
</div>
<div class="rule">
<h3 data-toc-exclude id="noNoninteractiveElementToInteractiveRole">
	<a href="/docs/lint/rules/jsx-a11y/noNoninteractiveElementToInteractiveRole">noNoninteractiveElementToInteractiveRole</a>
	<a class="header-anchor" href="#noNoninteractiveElementToInteractiveRole"></a>
</h3>
non-interactive elements should not be assigned interactive roles
</div>
<div class="rule">
<h3 data-toc-exclude id="noNoninteractiveTabindex">
	<a href="/docs/lint/rules/jsx-a11y/noNoninteractiveTabindex">noNoninteractiveTabindex</a>
	<a class="header-anchor" href="#noNoninteractiveTabindex"></a>
</h3>
&quot;`tabIndex` should only be declared on interactive elements&quot;
</div>
<div class="rule">
<h3 data-toc-exclude id="noOnChange">
	<a href="/docs/lint/rules/jsx-a11y/noOnChange">noOnChange</a>
	<a class="header-anchor" href="#noOnChange"></a>
</h3>
discourage the usage of `onChange`
</div>
<div class="rule">
<h3 data-toc-exclude id="noPositiveTabindex">
	<a href="/docs/lint/rules/jsx-a11y/noPositiveTabindex">noPositiveTabindex</a>
	<a class="header-anchor" href="#noPositiveTabindex"></a>
</h3>
enforce tabIndex value is not greater than zero
</div>
<div class="rule">
<h3 data-toc-exclude id="noRedundantAlt">
	<a href="/docs/lint/rules/jsx-a11y/noRedundantAlt">noRedundantAlt</a>
	<a class="header-anchor" href="#noRedundantAlt"></a>
</h3>
enforce `img` alt prop does not contain the word &quot;image&quot;, &quot;picture&quot;, or &quot;photo&quot;
</div>
<div class="rule">
<h3 data-toc-exclude id="noRedundantRoles">
	<a href="/docs/lint/rules/jsx-a11y/noRedundantRoles">noRedundantRoles</a>
	<a class="header-anchor" href="#noRedundantRoles"></a>
</h3>
enforce explicit role property is not the same as implicit/default role property on element
</div>
<div class="rule">
<h3 data-toc-exclude id="noTargetBlank">
	<a href="/docs/lint/rules/jsx-a11y/noTargetBlank">noTargetBlank</a>
	<a class="header-anchor" href="#noTargetBlank"></a>
</h3>
Prevent usage of unsafe `target=&quot;_blank&quot;`
</div>
<div class="rule">
<h3 data-toc-exclude id="useAltText">
	<a href="/docs/lint/rules/jsx-a11y/useAltText">useAltText</a>
	<a class="header-anchor" href="#useAltText"></a>
</h3>
enforce alternative text
</div>
<div class="rule">
<h3 data-toc-exclude id="useAnchorContent">
	<a href="/docs/lint/rules/jsx-a11y/useAnchorContent">useAnchorContent</a>
	<a class="header-anchor" href="#useAnchorContent"></a>
</h3>
enforce that anchors have content and that the content is accessible to screen readers
</div>
<div class="rule">
<h3 data-toc-exclude id="useAriaProps">
	<a href="/docs/lint/rules/jsx-a11y/useAriaProps">useAriaProps</a>
	<a class="header-anchor" href="#useAriaProps"></a>
</h3>
enforce all `aria-*` props are valid
</div>
<div class="rule">
<h3 data-toc-exclude id="useAriaPropsForRole">
	<a href="/docs/lint/rules/jsx-a11y/useAriaPropsForRole">useAriaPropsForRole</a>
	<a class="header-anchor" href="#useAriaPropsForRole"></a>
</h3>
enforce that elements with ARIA roles must have all required attributes for that role
</div>
<div class="rule">
<h3 data-toc-exclude id="useAriaProptypes">
	<a href="/docs/lint/rules/jsx-a11y/useAriaProptypes">useAriaProptypes</a>
	<a class="header-anchor" href="#useAriaProptypes"></a>
</h3>
enforce ARIA state and property values are valid
</div>
<div class="rule">
<h3 data-toc-exclude id="useHeadingContent">
	<a href="/docs/lint/rules/jsx-a11y/useHeadingContent">useHeadingContent</a>
	<a class="header-anchor" href="#useHeadingContent"></a>
</h3>
enforce heading (`h1`, `h2`, etc) elements contain accessible content
</div>
<div class="rule">
<h3 data-toc-exclude id="useHtmlLang">
	<a href="/docs/lint/rules/jsx-a11y/useHtmlLang">useHtmlLang</a>
	<a class="header-anchor" href="#useHtmlLang"></a>
</h3>
the `lang` attribute is mandatory
</div>
<div class="rule">
<h3 data-toc-exclude id="useIframeTitle">
	<a href="/docs/lint/rules/jsx-a11y/useIframeTitle">useIframeTitle</a>
	<a class="header-anchor" href="#useIframeTitle"></a>
</h3>
enforce `iframe` elements have a title attribute
</div>
<div class="rule">
<h3 data-toc-exclude id="useKeyWithClickEvents">
	<a href="/docs/lint/rules/jsx-a11y/useKeyWithClickEvents">useKeyWithClickEvents</a>
	<a class="header-anchor" href="#useKeyWithClickEvents"></a>
</h3>
enforce a clickable non-interactive element has at least one keyboard event listener.
</div>
<div class="rule">
<h3 data-toc-exclude id="useKeyWithMouseEvents">
	<a href="/docs/lint/rules/jsx-a11y/useKeyWithMouseEvents">useKeyWithMouseEvents</a>
	<a class="header-anchor" href="#useKeyWithMouseEvents"></a>
</h3>
enforce that `onMouseOver`/`onMouseOut` are accompanied by `onFocus`/`onBlur` for keyboard-only users
</div>
<div class="rule">
<h3 data-toc-exclude id="useMediaCaption">
	<a href="/docs/lint/rules/jsx-a11y/useMediaCaption">useMediaCaption</a>
	<a class="header-anchor" href="#useMediaCaption"></a>
</h3>
enforces that `audio` and `video` elements must have a `track` for captions
</div>
<div class="rule">
<h3 data-toc-exclude id="useValidAnchor">
	<a href="/docs/lint/rules/jsx-a11y/useValidAnchor">useValidAnchor</a>
	<a class="header-anchor" href="#useValidAnchor"></a>
</h3>
enforce all anchors are valid, navigable elements
</div>
<div class="rule">
<h3 data-toc-exclude id="useValidLang">
	<a href="/docs/lint/rules/jsx-a11y/useValidLang">useValidLang</a>
	<a class="header-anchor" href="#useValidLang"></a>
</h3>
check if `lang` attribute is valid
</div>
</section>
<section>
<h2>React</h2>
<p>Rule semantics and descriptions taken from <a href="https://github.com/yannickcr/eslint-plugin-react">eslint-plugin-react</a>. See individual rule docs for direct references.</p>
<div class="rule">
<h3 data-toc-exclude id="noAccessStateInSetState">
	<a href="/docs/lint/rules/react/noAccessStateInSetState">noAccessStateInSetState</a>
	<a class="header-anchor" href="#noAccessStateInSetState"></a>
</h3>
prevent using `this.state` within a `this.setState`
</div>
<div class="rule">
<h3 data-toc-exclude id="noArrayIndexKey">
	<a href="/docs/lint/rules/react/noArrayIndexKey">noArrayIndexKey</a>
	<a class="header-anchor" href="#noArrayIndexKey"></a>
</h3>
prevent usage of Array index in keys
</div>
<div class="rule">
<h3 data-toc-exclude id="noChildrenProp">
	<a href="/docs/lint/rules/react/noChildrenProp">noChildrenProp</a>
	<a class="header-anchor" href="#noChildrenProp"></a>
</h3>
prevent passing of children as props
</div>
<div class="rule">
<h3 data-toc-exclude id="noDanger">
	<a href="/docs/lint/rules/react/noDanger">noDanger</a>
	<a class="header-anchor" href="#noDanger"></a>
</h3>
prevent usage of dangerous JSX props
</div>
<div class="rule">
<h3 data-toc-exclude id="noDangerWithChildren">
	<a href="/docs/lint/rules/react/noDangerWithChildren">noDangerWithChildren</a>
	<a class="header-anchor" href="#noDangerWithChildren"></a>
</h3>
report when a DOM element is using both `children` and `dangerouslySetInnerHTML`
</div>
<div class="rule">
<h3 data-toc-exclude id="noDidMountSetState">
	<a href="/docs/lint/rules/react/noDidMountSetState">noDidMountSetState</a>
	<a class="header-anchor" href="#noDidMountSetState"></a>
</h3>
prevent usage of `setState` in `componentDidMount`
</div>
<div class="rule">
<h3 data-toc-exclude id="noDidUpdateSetState">
	<a href="/docs/lint/rules/react/noDidUpdateSetState">noDidUpdateSetState</a>
	<a class="header-anchor" href="#noDidUpdateSetState"></a>
</h3>
pevent usage of `setState` in `componentDidUpdate`
</div>
<div class="rule">
<h3 data-toc-exclude id="noDirectMutationState">
	<a href="/docs/lint/rules/react/noDirectMutationState">noDirectMutationState</a>
	<a class="header-anchor" href="#noDirectMutationState"></a>
</h3>
prevent direct mutation of `this.state`
</div>
<div class="rule">
<h3 data-toc-exclude id="noFindDOMNode">
	<a href="/docs/lint/rules/react/noFindDOMNode">noFindDOMNode</a>
	<a class="header-anchor" href="#noFindDOMNode"></a>
</h3>
prevent usage of `findDOMNode`
</div>
<div class="rule">
<h3 data-toc-exclude id="noRedundantShouldComponentUpdate">
	<a href="/docs/lint/rules/react/noRedundantShouldComponentUpdate">noRedundantShouldComponentUpdate</a>
	<a class="header-anchor" href="#noRedundantShouldComponentUpdate"></a>
</h3>
flag `shouldComponentUpdate` when extending `PureComponent`
</div>
<div class="rule">
<h3 data-toc-exclude id="noRenderReturnValue">
	<a href="/docs/lint/rules/react/noRenderReturnValue">noRenderReturnValue</a>
	<a class="header-anchor" href="#noRenderReturnValue"></a>
</h3>
prevent usage of the return value of `React.render`
</div>
<div class="rule">
<h3 data-toc-exclude id="noStringRefs">
	<a href="/docs/lint/rules/react/noStringRefs">noStringRefs</a>
	<a class="header-anchor" href="#noStringRefs"></a>
</h3>
prevent string definitions for references and prevent referencing `this.refs`
</div>
<div class="rule">
<h3 data-toc-exclude id="noThisInSFC">
	<a href="/docs/lint/rules/react/noThisInSFC">noThisInSFC</a>
	<a class="header-anchor" href="#noThisInSFC"></a>
</h3>
report `this` being used in stateless components
</div>
<div class="rule">
<h3 data-toc-exclude id="noUnsafe">
	<a href="/docs/lint/rules/react/noUnsafe">noUnsafe</a>
	<a class="header-anchor" href="#noUnsafe"></a>
</h3>
prevent usage of unsafe lifecycle methods
</div>
<div class="rule">
<h3 data-toc-exclude id="noUselessFragment">
	<a href="/docs/lint/rules/react/noUselessFragment">noUselessFragment</a>
	<a class="header-anchor" href="#noUselessFragment"></a>
</h3>
disallow unnecessary fragments
</div>
<div class="rule">
<h3 data-toc-exclude id="noVoidElementsWithChildren">
	<a href="/docs/lint/rules/react/noVoidElementsWithChildren">noVoidElementsWithChildren</a>
	<a class="header-anchor" href="#noVoidElementsWithChildren"></a>
</h3>
This rules prevent void elements from have children
</div>
<div class="rule">
<h3 data-toc-exclude id="noWillUpdateSetState">
	<a href="/docs/lint/rules/react/noWillUpdateSetState">noWillUpdateSetState</a>
	<a class="header-anchor" href="#noWillUpdateSetState"></a>
</h3>
prevent usage of `setState` in `componentWillUpdate`
</div>
<div class="rule">
<h3 data-toc-exclude id="useButtonType">
	<a href="/docs/lint/rules/react/useButtonType">useButtonType</a>
	<a class="header-anchor" href="#useButtonType"></a>
</h3>
Enforces a `type` attribute for `button`
</div>
<div class="rule">
<h3 data-toc-exclude id="useFragmentSyntax">
	<a href="/docs/lint/rules/react/useFragmentSyntax">useFragmentSyntax</a>
	<a class="header-anchor" href="#useFragmentSyntax"></a>
</h3>
This rule enforces the use of `&lt;&gt;...&lt;/&gt;`
</div>
<div class="rule">
<h3 data-toc-exclude id="useKey">
	<a href="/docs/lint/rules/react/useKey">useKey</a>
	<a class="header-anchor" href="#useKey"></a>
</h3>
This rule detects a missing `key` prop
</div>
<div class="rule">
<h3 data-toc-exclude id="useRenderReturn">
	<a href="/docs/lint/rules/react/useRenderReturn">useRenderReturn</a>
	<a class="header-anchor" href="#useRenderReturn"></a>
</h3>
This rule makes sure the render function is returning content
</div>
<div class="rule">
<h3 data-toc-exclude id="useSortComp">
	<a href="/docs/lint/rules/react/useSortComp">useSortComp</a>
	<a class="header-anchor" href="#useSortComp"></a>
</h3>
enforce component methods order
</div>
<div class="rule">
<h3 data-toc-exclude id="useStylePropObject">
	<a href="/docs/lint/rules/react/useStylePropObject">useStylePropObject</a>
	<a class="header-anchor" href="#useStylePropObject"></a>
</h3>
enforce style prop value is an object
</div>
</section>
<!-- GENERATED:END(id:main) -->
