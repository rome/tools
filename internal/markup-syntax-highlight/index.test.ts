import {TestHelper, test} from "rome";
import "@internal/core";
import {createUnknownPath} from "@internal/path";
import {DiagnosticLanguage} from "@internal/diagnostics";
import {
	AnsiHighlightOptions,
	highlightCode,
} from "@internal/markup-syntax-highlight";
import {AnyMarkups} from "@internal/markup/escape";

test(
	"should highlight JS",
	testCase(
		"js",
		`
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
		(<AnyMarkups>[
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "",
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '<token type="keyword">function</token> <token type="keyword">async</token> <token type="function">foo</token><token type="punctuation">(</token><token type="variable">bar</token><token type="punctuation">)</token> <token type="punctuation">{</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t<token type="keyword">for</token> <token type="punctuation">(</token><token type="keyword">let</token> <token type="variable">i</token> <token type="operator">=</token> <token type="number">0</token><token type="punctuation">;</token> <token type="variable">i</token> <token type="operator">\\<</token> <token type="variable">bar</token><token type="punctuation">.</token><token type="variable">length</token><token type="punctuation">;</token> <token type="operator">++</token><token type="variable">i</token><token type="punctuation">)</token> <token type="punctuation">{</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t\t<token type="keyword">if</token> <token type="punctuation">(</token><token type="variable">bar</token><token type="punctuation">[</token><token type="variable">i</token><token type="punctuation">]</token> <token type="operator">===</token> <token type="number">123321</token><token type="punctuation">)</token> <token type="punctuation">{</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t\t\t<token type="keyword">return</token> <token type="boolean">true</token><token type="punctuation">;</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t\t<token type="punctuation">}</token> <token type="keyword">else</token> <token type="punctuation">(</token><token type="variable">bar</token><token type="punctuation">[</token><token type="variable">i</token><token type="punctuation">]</token> <token type="operator">===</token> <token type="string">`</token><token type="string">template string</token><token type="string">`</token><token type="punctuation">)</token> <token type="punctuation">{</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t\t\t<token type="keyword">return</token> <token type="boolean">false</token><token type="punctuation">;</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t\t<token type="punctuation">}</token> <token type="keyword">else</token> <token type="punctuation">{</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t\t\t<token type="keyword">return</token> \\<<token type="variable">Compenent</token> <token type="attr-name">attribute</token><token type="operator">=</token><token type="string">\\"JSX content\\"</token>>Test\\<<token type="operator">/</token><token type="variable">Component</token>><token type="punctuation">;</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t\t<token type="punctuation">}</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t\t<token type="variable">_something_invalid_</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t\t<token type="comment">/* This is a comment */</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t<token type="punctuation">}</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "",
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t<token type="keyword">return</token> <token type="punctuation">(</token><token type="variable">a</token><token type="punctuation">,</token> <token type="variable">b</token><token type="punctuation">)</token> <token type="operator">=></token> <token type="punctuation">{</token> <token type="keyword">return</token> <token type="variable">a</token> <token type="operator">+</token> <token type="variable">b</token><token type="punctuation">;</token> <token type="punctuation">}</token><token type="punctuation">;</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '<token type="punctuation">}</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "",
					},
				],
			},
		]),
	),
);

test(
	"should highlight HTML",
	testCase(
		"html",
		`
<img src="https://example.com/image.png"/>
<p class="test-class another_test_class">This is some text</p>
<div class="test-class another_test_class">
	<p class="test-class">This is a nested element</p>
	This is some nested text
	<p class="test-class">This is another nested element</p>
</div>
`,
		(<AnyMarkups>[
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "",
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '<token type="punctuation">\\<</token><token type="tag">img</token> <token type="attr-name">src</token><token type="attr-equals">=</token><token type="attr-value">\\"https://example.com/image.png\\"</token><token type="punctuation">/></token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '<token type="punctuation">\\<</token><token type="tag">p</token> <token type="attr-name">class</token><token type="attr-equals">=</token><token type="attr-value">\\"test-class another_test_class\\"</token><token type="punctuation">></token>This is some text<token type="punctuation">\\</</token><token type="attr-name">p</token><token type="punctuation">></token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '<token type="punctuation">\\<</token><token type="tag">div</token> <token type="attr-name">class</token><token type="attr-equals">=</token><token type="attr-value">\\"test-class another_test_class\\"</token><token type="punctuation">></token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t<token type="punctuation">\\<</token><token type="tag">p</token> <token type="attr-name">class</token><token type="attr-equals">=</token><token type="attr-value">\\"test-class\\"</token><token type="punctuation">></token>This is a nested element<token type="punctuation">\\</</token><token type="attr-name">p</token><token type="punctuation">></token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "\tThis is some nested text",
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t<token type="punctuation">\\<</token><token type="tag">p</token> <token type="attr-name">class</token><token type="attr-equals">=</token><token type="attr-value">\\"test-class\\"</token><token type="punctuation">></token>This is another nested element<token type="punctuation">\\</</token><token type="attr-name">p</token><token type="punctuation">></token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '<token type="punctuation">\\</</token><token type="attr-name">div</token><token type="punctuation">></token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "",
					},
				],
			},
		]),
	),
);

test(
	"should highlight JSON",
	testCase(
		"json",
		`
{
	"value": "content",
	"another_value": 123,
	"key": true
	// Line comment
	/* Block Comment */
}
`,
		(<AnyMarkups>[
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "",
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '<token type="punctuation">{</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t<token type="string">\\"value\\"</token><token type="operator">:</token> <token type="string">\\"content\\"</token><token type="operator">,</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t<token type="string">\\"another_value\\"</token><token type="operator">:</token> <token type="number">123</token><token type="operator">,</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t<token type="string">\\"key\\"</token><token type="operator">:</token> <token type="boolean">true</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t<token type="comment">// Line comment</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\t<token type="comment">/* Block Comment */</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '<token type="punctuation">}</token>',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "",
					},
				],
			},
		]),
	),
);

test(
	"should highlight SHELL",
	testCase(
		"shell",
		`
#!/bin/bash 
export bar="string"
function foo {
	ls -la /simple/path
	ls -la "/path with/spaces" | grep -e "something"
}
`,
		(<AnyMarkups>[
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '<token type="function">',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "#!/bin/bash</token> <dim>",
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": 'export</dim> <dim>bar=\\"string\\"',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "function</dim> <dim>foo</dim> <dim>{",
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "\tls</dim> <dim>-la</dim> <dim>/simple/path",
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": '\tls</dim> <dim>-la</dim> <token type="string">\\"/path</token> <dim>with/spaces\\"</dim> <dim>|</dim> <dim>grep</dim> <dim>-e</dim> <token type="string">\\"something\\"',
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "}",
					},
				],
			},
			{
				"type": "MARKUP",
				"parts": [
					{
						"type": "RAW_MARKUP",
						"value": "</token>",
					},
				],
			},
		]),
	),
);

function testCase(
	language: DiagnosticLanguage,
	input: string,
	expectedOutput: AnyMarkups,
) {
	return async (t: TestHelper) => {
		const highlighted = highlightCode(craftTestInput(input, language));
		t.looksLike(highlighted, expectedOutput);
	};
}

function craftTestInput(input: string, language: DiagnosticLanguage) {
	return (<AnsiHighlightOptions>{
		path: createUnknownPath("/unknown"),
		input,
		sourceTypeJS: undefined,
		language,
		highlight: true,
	});
}
