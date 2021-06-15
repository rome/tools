/* GENERATED:START(hash:de3173477f8285558cafa004bea700bf0ec2462f,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
import {Rules} from "@internal/project/lint";

export function mergeRules(
	a: Rules | undefined,
	b: Rules | undefined,
): Rules | undefined {
	if (!a) {
		return b;
	}
	if (!b) {
		return a;
	}
	let rules: Rules | undefined;
	if (a.recommended || b.recommended) {
		rules = {
			recommended: true,
		};
	} else {
		rules = {};
		if (!a.a11y) {
			rules.a11y = b.a11y;
		} else if (!b.a11y) {
			rules.a11y = a.a11y;
		} else {
			rules.a11y = new Map([...a.a11y.entries(), ...b.a11y.entries()]);
		}
		if (!a.css) {
			rules.css = b.css;
		} else if (!b.css) {
			rules.css = a.css;
		} else {
			rules.css = new Map([...a.css.entries(), ...b.css.entries()]);
		}
		if (!a.html) {
			rules.html = b.html;
		} else if (!b.html) {
			rules.html = a.html;
		} else {
			rules.html = new Map([...a.html.entries(), ...b.html.entries()]);
		}
		if (!a.js) {
			rules.js = b.js;
		} else if (!b.js) {
			rules.js = a.js;
		} else {
			rules.js = new Map([...a.js.entries(), ...b.js.entries()]);
		}
		if (!a.jsx) {
			rules.jsx = b.jsx;
		} else if (!b.jsx) {
			rules.jsx = a.jsx;
		} else {
			rules.jsx = new Map([...a.jsx.entries(), ...b.jsx.entries()]);
		}
		if (!a.react) {
			rules.react = b.react;
		} else if (!b.react) {
			rules.react = a.react;
		} else {
			rules.react = new Map([...a.react.entries(), ...b.react.entries()]);
		}
		if (!a.regex) {
			rules.regex = b.regex;
		} else if (!b.regex) {
			rules.regex = a.regex;
		} else {
			rules.regex = new Map([...a.regex.entries(), ...b.regex.entries()]);
		}
		if (!a.ts) {
			rules.ts = b.ts;
		} else if (!b.ts) {
			rules.ts = a.ts;
		} else {
			rules.ts = new Map([...a.ts.entries(), ...b.ts.entries()]);
		}
	}

	return rules;
}
/* GENERATED:END(id:main) */
