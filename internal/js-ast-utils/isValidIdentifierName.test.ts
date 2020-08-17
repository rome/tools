import {test} from "rome";
import {isValidIdentifierName} from "@internal/js-ast-utils/isValidIdentifierName";

test(
	"invalid identifiers",
	async (t) => {
		let invalid = [
			"",
			" ",
			"await",
			"enum",
			"implements",
			"interface",
			"let",
			"package",
			"private",
			"protected",
			"public",
			"static",
			"yield",
			"eval",
			"arguments",
			"break",
			"case",
			"catch",
			"continue",
			"debugger",
			"default",
			"do",
			"else",
			"finally",
			"for",
			"function",
			"if",
			"return",
			"switch",
			"throw",
			"try",
			"var",
			"const",
			"while",
			"with",
			"new",
			"this",
			"super",
			"class",
			"extends",
			"export",
			"import",
			"null",
			"true",
			"false",
			"in",
			"instanceof",
			"typeof",
			"void",
			"delete",
			"`",
			"{",
			"\xa9",
			"\u{1d11e}",
			"\u{1d401}",
			"\u{1f035}",
			"\u{1f35c}",
			"\u{1f602}",
			String.fromCharCode(65_536),
		];

		invalid.map((el) => {
			t.false(isValidIdentifierName(el));
		});
	},
);

test(
	"valid identifiers",
	async (t) => {
		const valid = [
			"$",
			"_",
			"\xaa",
			"\xb5",
			String.fromCharCode(2_154),
			String.fromCharCode(3_314),
			String.fromCharCode(8_584),
			String.fromCharCode(42_606),
			String.fromCharCode(43_642),
			String.fromCharCode(65_500),
			"A",
			"Z",
			"a",
			"z",
			"hello",
			"world",
			"awaited",
			"deleted",
			"istypeof",
			"truest",
			"nullish",
			"dontcontinue",
			"mypackage",
			"incase",
			"export_",
			"_import",
			"$this",
		];

		valid.map((el) => {
			t.true(isValidIdentifierName(el));
		});
	},
);
