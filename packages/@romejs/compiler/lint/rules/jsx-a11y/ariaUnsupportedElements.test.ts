import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"aria unsupported elements",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<meta charset='UTF-8' aria-hidden='false' />",
				"<meta charset='UTF-8' role='meta' />",
				"<html aria-required='true' />",
				"<html role='html'></html>",
				"<script aria-label='script'></script>",
				"<script role='script'></script>",
				"<style aria-labelledby></style>",
				"<style role='style'></style>",
				// VALID
				"<meta charset='UTF-8' />",
				"<html maria='text'></html>",
				"<script></script>",
				"<style parole></style>",
			],
			{category: "lint/jsx-a11y/ariaUnsupportedElements"},
		);
	},
);
