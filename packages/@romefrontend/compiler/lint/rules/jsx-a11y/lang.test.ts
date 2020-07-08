import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y lang test",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					'<html lang="foo"></html>',
					'<html lang="ex"></html>',
					'<html lang="foo-bar"></html>',
					'<html lang="aa-zz"></html>',
					'<html lang="zz-AA"></html>',
					'<html lang="en2></html>',
				],
				valid: [
					'<html lang="en-US"></html>',
					'<html lang="en"></html>',
					"<html lang={lang}></html>",
				],
			},
			{category: "lint/jsx-a11y/lang"},
		);
	},
);
