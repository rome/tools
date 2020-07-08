import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y aria unsupported elements",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<meta charset='UTF-8' aria-hidden='false' />",
					"<meta charset='UTF-8' role='meta' />",
					"<html aria-required='true' />",
					"<html role='html'></html>",
					"<script aria-label='script'></script>",
					"<script role='script'></script>",
					"<style aria-labelledby></style>",
					"<style role='style'></style>",
				],
				valid: [
					"<meta charset='UTF-8' />",
					"<html maria='text'></html>",
					"<script></script>",
					"<style parole></style>",
				],
			},
			{category: "lint/jsx-a11y/ariaUnsupportedElements"},
		);
	},
);
