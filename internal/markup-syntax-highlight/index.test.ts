import {TestHelper, test} from "rome";
import "@internal/core";
import {UNKNOWN_PATH} from "@internal/path";
import {DiagnosticLanguage} from "@internal/diagnostics";
import {
	AnsiHighlightOptions,
	highlightCode,
} from "@internal/markup-syntax-highlight";
import {concatMarkup, markup, readMarkup} from "@internal/markup";
import {dedent} from "@internal/string-utils";

test(
	"should highlight JS",
	testCase(
		"js",
		dedent`
			function async foo(bar) {
				for (let i = 0; i < bar.length; ++i) {
					if (bar[i] === 123321) {
						return true;
					} else (bar[i] === \`template string\`) {
						return false;
					} else {
						return <Compenent attribute="JSX content">Test</Component>;
					}
					_something_invalid_
					/* This is a comment */
				}

				return (a, b) => { return a + b; };
			}
		`,
		(t, markup) =>
			t.inlineSnapshot(
				markup,
				'<token type="keyword">function</token> <token type="keyword">async</token> <token type="function">foo</token><token type="punctuation">(</token><token type="variable">bar</token><token type="punctuation">)</token> <token type="punctuation">{</token>\n\t<token type="keyword">for</token> <token type="punctuation">(</token><token type="keyword">let</token> <token type="variable">i</token> <token type="operator">=</token> <token type="number">0</token><token type="punctuation">;</token> <token type="variable">i</token> <token type="operator">\\<</token> <token type="variable">bar</token><token type="punctuation">.</token><token type="variable">length</token><token type="punctuation">;</token> <token type="operator">++</token><token type="variable">i</token><token type="punctuation">)</token> <token type="punctuation">{</token>\n\t\t<token type="keyword">if</token> <token type="punctuation">(</token><token type="variable">bar</token><token type="punctuation">[</token><token type="variable">i</token><token type="punctuation">]</token> <token type="operator">===</token> <token type="number">123321</token><token type="punctuation">)</token> <token type="punctuation">{</token>\n\t\t\t<token type="keyword">return</token> <token type="boolean">true</token><token type="punctuation">;</token>\n\t\t<token type="punctuation">}</token> <token type="keyword">else</token> <token type="punctuation">(</token><token type="variable">bar</token><token type="punctuation">[</token><token type="variable">i</token><token type="punctuation">]</token> <token type="operator">===</token> <token type="variable">\\\\`template</token> <token type="variable">string\\\\`</token><token type="punctuation">)</token> <token type="punctuation">{</token>\n\t\t\t<token type="keyword">return</token> <token type="boolean">false</token><token type="punctuation">;</token>\n\t\t<token type="punctuation">}</token> <token type="keyword">else</token> <token type="punctuation">{</token>\n\t\t\t<token type="keyword">return</token> \\<<token type="variable">Compenent</token> <token type="attr-name">attribute</token><token type="operator">=</token><token type="string">\\"JSX content\\"</token>>Test\\<<token type="operator">/</token><token type="variable">Component</token>><token type="punctuation">;</token>\n\t\t<token type="punctuation">}</token>\n\t\t<token type="variable">_something_invalid_</token>\n\t\t<token type="comment">/* This is a comment */</token>\n\t<token type="punctuation">}</token>\n\n\t<token type="keyword">return</token> <token type="punctuation">(</token><token type="variable">a</token><token type="punctuation">,</token> <token type="variable">b</token><token type="punctuation">)</token> <token type="operator">=></token> <token type="punctuation">{</token> <token type="keyword">return</token> <token type="variable">a</token> <token type="operator">+</token> <token type="variable">b</token><token type="punctuation">;</token> <token type="punctuation">}</token><token type="punctuation">;</token>\n<token type="punctuation">}</token>',
			)
		,
	),
);

test(
	"should highlight HTML",
	testCase(
		"html",
		dedent`
			<img src="https://example.com/image.png"/>
			<p class="test-class another_test_class">This is some text</p>
			<div class="test-class another_test_class">
				<p class="test-class">This is a nested element</p>
				This is some nested text
				<p class="test-class">This is another nested element</p>
			</div>
		`,
		(t, markup) =>
			t.inlineSnapshot(
				markup,
				'<token type="punctuation">\\<</token><token type="tag">img</token> <token type="attr-name">src</token><token type="attr-equals">=</token><token type="attr-value">\\"https://example.com/image.png\\"</token><token type="punctuation">/></token>\n<token type="punctuation">\\<</token><token type="tag">p</token> <token type="attr-name">class</token><token type="attr-equals">=</token><token type="attr-value">\\"test-class another_test_class\\"</token><token type="punctuation">></token>This is some text<token type="punctuation">\\</</token><token type="attr-name">p</token><token type="punctuation">></token>\n<token type="punctuation">\\<</token><token type="tag">div</token> <token type="attr-name">class</token><token type="attr-equals">=</token><token type="attr-value">\\"test-class another_test_class\\"</token><token type="punctuation">></token>\n\t<token type="punctuation">\\<</token><token type="tag">p</token> <token type="attr-name">class</token><token type="attr-equals">=</token><token type="attr-value">\\"test-class\\"</token><token type="punctuation">></token>This is a nested element<token type="punctuation">\\</</token><token type="attr-name">p</token><token type="punctuation">></token>\n\tThis is some nested text\n\t<token type="punctuation">\\<</token><token type="tag">p</token> <token type="attr-name">class</token><token type="attr-equals">=</token><token type="attr-value">\\"test-class\\"</token><token type="punctuation">></token>This is another nested element<token type="punctuation">\\</</token><token type="attr-name">p</token><token type="punctuation">></token>\n<token type="punctuation">\\</</token><token type="attr-name">div</token><token type="punctuation">></token>',
			)
		,
	),
);

test(
	"should highlight JSON",
	testCase(
		"json",
		dedent`
			{
				"value": "content",
				"another_value": 123,
				"key": true
				// Line comment
				/* Block Comment */
			}
		`,
		(t, markup) =>
			t.inlineSnapshot(
				markup,
				'<token type="punctuation">{</token>\n\t<token type="string">\\"value\\"</token><token type="operator">:</token> <token type="string">\\"content\\"</token><token type="operator">,</token>\n\t<token type="string">\\"another_value\\"</token><token type="operator">:</token> <token type="number">123</token><token type="operator">,</token>\n\t<token type="string">\\"key\\"</token><token type="operator">:</token> <token type="boolean">true</token>\n\t<token type="comment">// Line comment</token>\n\t<token type="comment">/* Block Comment */</token>\n<token type="punctuation">}</token>',
			)
		,
	),
);

test(
	"should highlight SHELL",
	testCase(
		"shell",
		dedent`
			#!/bin/bash
			export bar="string"
			function foo {
				ls -la /simple/path
				ls -la "/path with/spaces" | grep -e "something"
			}
		`,
		(t, markup) =>
			t.inlineSnapshot(
				markup,
				'<token type="function">#!/bin/bash\nexport</token> <dim>bar=\\"string\\"\nfunction</dim> <dim>foo</dim> <dim>{\n\tls</dim> <dim>-la</dim> <dim>/simple/path\n\tls</dim> <dim>-la</dim> <token type="string">\\"/path</token> <dim>with/spaces\\"</dim> <dim>|</dim> <dim>grep</dim> <dim>-e</dim> <token type="string">\\"something\\"\n}</token>',
			)
		,
	),
);

function testCase(
	language: DiagnosticLanguage,
	input: string,
	callback: (t: TestHelper, markup: string) => void,
) {
	return async (t: TestHelper) => {
		const highlighted = highlightCode(craftTestInput(input, language));
		callback(t, readMarkup(concatMarkup(highlighted, markup`\n`)));
	};
}

function craftTestInput(
	input: string,
	language: DiagnosticLanguage,
): AnsiHighlightOptions {
	return {
		path: UNKNOWN_PATH,
		input,
		sourceTypeJS: undefined,
		language,
		highlight: true,
	};
}
