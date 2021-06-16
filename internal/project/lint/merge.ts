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

	if (a.recommended || b.recommended) {
		return {
			recommended: true,
		};
	} else {
		return {
			...a,
			...b,
		};
	}
}
/* GENERATED:END(id:main) */
