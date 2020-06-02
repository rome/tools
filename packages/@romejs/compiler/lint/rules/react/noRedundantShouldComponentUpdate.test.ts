import {test} from "rome";
import {testLint} from "../testHelpers";
import {dedent} from "@romejs/string-utils";

test(
	"react no redundant should component update",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						class Hello extends React.PureComponent {
							shouldComponentUpdate() {}
						}
					`,
					dedent`
						class Hello extends PureComponent {
							shouldComponentUpdate() {}
						}
					`,
				],
				valid: [
					dedent`
						class Hello extends React.PureComponent {
							componentDidMount() {}
						}
					`,
					dedent`
						class Hello extends PureComponent {
							componentDidMount() {}
						}
					`,
				],
			},
			{category: "lint/react/noRedundantShouldComponentUpdate"},
		);
	},
);
