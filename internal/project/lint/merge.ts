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
		if (a.a11y) {
			if (b.a11y) {
				if (typeof a.a11y === "boolean" && typeof b.a11y === "boolean") {
					// b takes over
					rules.a11y = b.a11y;
				} else if (typeof a.a11y !== "boolean" && typeof b.a11y !== "boolean") {
					rules.a11y = new Map([...a.a11y.entries(), ...b.a11y.entries()]);
				} else {
					// b takes over
					rules.a11y = b.a11y;
				}
			} else {
				rules.a11y = a.a11y;
			}
		} else {
			rules.a11y = b.a11y;
		}
		if (a.css) {
			if (b.css) {
				if (typeof a.css === "boolean" && typeof b.css === "boolean") {
					// b takes over
					rules.css = b.css;
				} else if (typeof a.css !== "boolean" && typeof b.css !== "boolean") {
					rules.css = new Map([...a.css.entries(), ...b.css.entries()]);
				} else {
					// b takes over
					rules.css = b.css;
				}
			} else {
				rules.css = a.css;
			}
		} else {
			rules.css = b.css;
		}
		if (a.html) {
			if (b.html) {
				if (typeof a.html === "boolean" && typeof b.html === "boolean") {
					// b takes over
					rules.html = b.html;
				} else if (typeof a.html !== "boolean" && typeof b.html !== "boolean") {
					rules.html = new Map([...a.html.entries(), ...b.html.entries()]);
				} else {
					// b takes over
					rules.html = b.html;
				}
			} else {
				rules.html = a.html;
			}
		} else {
			rules.html = b.html;
		}
		if (a.js) {
			if (b.js) {
				if (typeof a.js === "boolean" && typeof b.js === "boolean") {
					// b takes over
					rules.js = b.js;
				} else if (typeof a.js !== "boolean" && typeof b.js !== "boolean") {
					rules.js = new Map([...a.js.entries(), ...b.js.entries()]);
				} else {
					// b takes over
					rules.js = b.js;
				}
			} else {
				rules.js = a.js;
			}
		} else {
			rules.js = b.js;
		}
		if (a.jsx) {
			if (b.jsx) {
				if (typeof a.jsx === "boolean" && typeof b.jsx === "boolean") {
					// b takes over
					rules.jsx = b.jsx;
				} else if (typeof a.jsx !== "boolean" && typeof b.jsx !== "boolean") {
					rules.jsx = new Map([...a.jsx.entries(), ...b.jsx.entries()]);
				} else {
					// b takes over
					rules.jsx = b.jsx;
				}
			} else {
				rules.jsx = a.jsx;
			}
		} else {
			rules.jsx = b.jsx;
		}
		if (a.react) {
			if (b.react) {
				if (typeof a.react === "boolean" && typeof b.react === "boolean") {
					// b takes over
					rules.react = b.react;
				} else if (typeof a.react !== "boolean" && typeof b.react !== "boolean") {
					rules.react = new Map([...a.react.entries(), ...b.react.entries()]);
				} else {
					// b takes over
					rules.react = b.react;
				}
			} else {
				rules.react = a.react;
			}
		} else {
			rules.react = b.react;
		}
		if (a.regex) {
			if (b.regex) {
				if (typeof a.regex === "boolean" && typeof b.regex === "boolean") {
					// b takes over
					rules.regex = b.regex;
				} else if (typeof a.regex !== "boolean" && typeof b.regex !== "boolean") {
					rules.regex = new Map([...a.regex.entries(), ...b.regex.entries()]);
				} else {
					// b takes over
					rules.regex = b.regex;
				}
			} else {
				rules.regex = a.regex;
			}
		} else {
			rules.regex = b.regex;
		}
		if (a.ts) {
			if (b.ts) {
				if (typeof a.ts === "boolean" && typeof b.ts === "boolean") {
					// b takes over
					rules.ts = b.ts;
				} else if (typeof a.ts !== "boolean" && typeof b.ts !== "boolean") {
					rules.ts = new Map([...a.ts.entries(), ...b.ts.entries()]);
				} else {
					// b takes over
					rules.ts = b.ts;
				}
			} else {
				rules.ts = a.ts;
			}
		} else {
			rules.ts = b.ts;
		}
	}

	return rules;
}
/* GENERATED:END(id:main) */
