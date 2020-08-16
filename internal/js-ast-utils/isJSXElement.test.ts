import {test} from "rome";
import {isJSXElement} from "./isJSXElement";
import {template} from "@internal/js-ast-utils";

test(
	"returns true for jsx elements",
	(t) => {
		t.true(isJSXElement(template.expression`<div>a</div>`, "div"));
		t.true(isJSXElement(template.expression`<ul></ul>`, "ul"));
		t.true(isJSXElement(template.expression`<ul>text</ul>`, "ul"));
		t.true(
			isJSXElement(template.expression`<span id="key">{var}</span>`, "span"),
		);
		t.true(
			isJSXElement(template.expression`<CustomComp></CustomComp>`, "CustomComp"),
		);
		t.true(isJSXElement(template.expression`<component>`, "component"));
	},
);

test(
	"returns false for non-jsx elements",
	(t) => {
		t.false(isJSXElement(template.expression`2+3`, "div"));
		t.false(isJSXElement(template.expression`someValue / 4`, "ul"));
		t.false(isJSXElement(template.expression`2;`, "li"));
		t.false(isJSXElement(template.expression`true`, "span"));
		t.false(isJSXElement(template.statement`if(a>2){}`, "li"));
	},
);

test(
	"returns false when there is name mismatch",
	(t) => {
		t.false(isJSXElement(template.expression`<div></div>`, "ul"));
		t.false(isJSXElement(template.expression`<span></span>`, "img"));
		t.false(isJSXElement(template.expression`<test>`, "otherTest"));
	},
);
