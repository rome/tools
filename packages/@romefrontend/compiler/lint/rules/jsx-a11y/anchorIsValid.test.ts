import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y anchor is valid",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<a href='#' onClick={foo} />",
					"<a href={'#'} onClick={foo} />",
					"<a href={`#`} onClick={foo} />",
					"<a onClick={foo} />",
					"<a href='javascript:void(0)' onClick={foo} />",
					"<a href={'javascript:void(0)'} onClick={foo} />",
					"<a href={`javascript:void(0)`} onClick={foo} />",
					"<a />",
					"<a href={undefined} />",
					"<a href={null} />",
					"<a href='#' />",
					"<a href={'#'} />",
					"<a href={`#`} />",
					"<a href='javascript:void(0)' />",
					"<a href={'javascript:void(0)'} />",
					"<a href={`javascript:void(0)`} />",
				],
				valid: [
					"<a href='https://github.com' />",
					"<a href='#section' />",
					"<a href='foo' />",
					"<a href='/foo/bar' />",
					"<a href={someValidPath} />",
					"<a href='https://github.com' onClick={foo} />",
					"<a href='#section' onClick={foo} />",
					"<a href='foo' onClick={foo} />",
					"<a href='/foo/bar' onClick={foo} />",
					"<a href={someValidPath} onClick={foo} />",
				],
				category: "lint/jsx-a11y/anchorIsValid",
				filename: "file.tsx",
			},
		);
	},
);
