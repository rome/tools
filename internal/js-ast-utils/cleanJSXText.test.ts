import {test} from "rome";
import {cleanJSXText} from "@internal/js-ast-utils/cleanJSXText";

test(
	"Verify cleaned JSX Text",
	async (t) => {
		t.is(
			cleanJSXText(
				`

		Hello

		world


		`,
			),
			"Hello world",
		);

		t.is(cleanJSXText("foo\t\t\tbar"), "foo   bar");
	},
);
