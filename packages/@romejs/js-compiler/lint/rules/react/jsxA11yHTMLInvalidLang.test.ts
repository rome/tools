import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"should warn when an invalid lang is provided",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				'<html lang="foo"></html>',
				'<html lang="foo-bar"></html>',
				'<html lang="aa-zz"></html>',
				'<html lang="zz-AA"></html>',
				// VALID
				'<html lang="en-US"></html>',
			],
			{category: "lint/jsxA11yHTMLInvalidLang"},
		);
	},
);
