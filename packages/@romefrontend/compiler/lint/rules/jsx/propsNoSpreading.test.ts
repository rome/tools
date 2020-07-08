import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"react jsx props no spreading",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<App {...props} />",
					"<MyCustomComponent {...props} some_other_prop={some_other_prop} />",
					"<MyCustomComponent some_other_prop={some_other_prop} {...props} />",
					"<img {...props} />",
				],
				valid: [
					"<MyCustomComponent one_prop={one_prop} two_prop={two_prop} />",
					"<img src={src} alt={alt} />",
				],
			},
			{category: "lint/jsx/propsNoSpreading"},
		);
	},
);
