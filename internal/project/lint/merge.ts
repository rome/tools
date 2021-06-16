/* GENERATED:START(hash:8c6d773b596209b59d5fe5c22f571367d047a4ac,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
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
			if (typeof a.a11y === "boolean" && typeof b.a11y === "boolean") {
				// b takes over
				rules.a11y = b.a11y;
			} else if (typeof a.a11y !== "boolean" && typeof b.a11y !== "boolean") {
				rules.a11y = new Map([...a.a11y.entries(), ...b.a11y.entries()]);
			} else {
				// b takes over
				rules.a11y = b.a11y;
			}
		}
		if (!a.css) {
			rules.css = b.css;
		} else if (!b.css) {
			rules.css = a.css;
		} else {
			if (typeof a.css === "boolean" && typeof b.css === "boolean") {
				// b takes over
				rules.css = b.css;
			} else if (typeof a.css !== "boolean" && typeof b.css !== "boolean") {
				rules.css = new Map([...a.css.entries(), ...b.css.entries()]);
			} else {
				// b takes over
				rules.css = b.css;
			}
		}
		if (!a.html) {
			rules.html = b.html;
		} else if (!b.html) {
			rules.html = a.html;
		} else {
			if (typeof a.html === "boolean" && typeof b.html === "boolean") {
				// b takes over
				rules.html = b.html;
			} else if (typeof a.html !== "boolean" && typeof b.html !== "boolean") {
				rules.html = new Map([...a.html.entries(), ...b.html.entries()]);
			} else {
				// b takes over
				rules.html = b.html;
			}
		}
		if (!a.js) {
			rules.js = b.js;
		} else if (!b.js) {
			rules.js = a.js;
		} else {
			if (typeof a.js === "boolean" && typeof b.js === "boolean") {
				// b takes over
				rules.js = b.js;
			} else if (typeof a.js !== "boolean" && typeof b.js !== "boolean") {
				rules.js = new Map([...a.js.entries(), ...b.js.entries()]);
			} else {
				// b takes over
				rules.js = b.js;
			}
		}
		if (!a.jsx) {
			rules.jsx = b.jsx;
		} else if (!b.jsx) {
			rules.jsx = a.jsx;
		} else {
			if (typeof a.jsx === "boolean" && typeof b.jsx === "boolean") {
				// b takes over
				rules.jsx = b.jsx;
			} else if (typeof a.jsx !== "boolean" && typeof b.jsx !== "boolean") {
				rules.jsx = new Map([...a.jsx.entries(), ...b.jsx.entries()]);
			} else {
				// b takes over
				rules.jsx = b.jsx;
			}
		}
		if (!a.react) {
			rules.react = b.react;
		} else if (!b.react) {
			rules.react = a.react;
		} else {
			if (typeof a.react === "boolean" && typeof b.react === "boolean") {
				// b takes over
				rules.react = b.react;
			} else if (typeof a.react !== "boolean" && typeof b.react !== "boolean") {
				rules.react = new Map([...a.react.entries(), ...b.react.entries()]);
			} else {
				// b takes over
				rules.react = b.react;
			}
		}
		if (!a.regex) {
			rules.regex = b.regex;
		} else if (!b.regex) {
			rules.regex = a.regex;
		} else {
			if (typeof a.regex === "boolean" && typeof b.regex === "boolean") {
				// b takes over
				rules.regex = b.regex;
			} else if (typeof a.regex !== "boolean" && typeof b.regex !== "boolean") {
				rules.regex = new Map([...a.regex.entries(), ...b.regex.entries()]);
			} else {
				// b takes over
				rules.regex = b.regex;
			}
		}
		if (!a.ts) {
			rules.ts = b.ts;
		} else if (!b.ts) {
			rules.ts = a.ts;
		} else {
			if (typeof a.ts === "boolean" && typeof b.ts === "boolean") {
				// b takes over
				rules.ts = b.ts;
			} else if (typeof a.ts !== "boolean" && typeof b.ts !== "boolean") {
				rules.ts = new Map([...a.ts.entries(), ...b.ts.entries()]);
			} else {
				// b takes over
				rules.ts = b.ts;
			}
		}
	}

	return rules;
}
/* GENERATED:END(id:main) */
