import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"jsx-a11y has lang",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<html></html>",
					"<html {...props}></html>",
					'<html lang=""></html>',
					'<html lang={""}></html>',
					"<html lang={``}></html>",
					"<html lang={undefined}></html>",
					"<html lang={false}></html>",
					"<html lang={true}></html>",
					"<html lang={42}></html>",
				],
				valid: [
					'<html lang="en"></html>',
					"<html lang={language}></html>",
					"<html lang={() => language}></html>",
				],
			},
			{category: "lint/jsx-a11y/htmlHasLang"},
		);
	},
);
