import {test} from "rome";
import {testLint} from "../testHelpers";
import {dedent} from "@romejs/string-utils";

test(
	"react no unsafe",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						class Hello extends React.Component {
							UNSAFE_componentWillMount() {}
						}
					`,
					dedent`
						class Hello extends React.Component {
							UNSAFE_componentWillReceiveProps() {}
						}
					`,
					dedent`
						class Hello extends React.Component {
							UNSAFE_componentWillUpdate() {}
						}
					`,
				],
				valid: [
					dedent`
						class Hello extends React.Component {
							componentDidMount() {}
						}
					`,
				],
			},
			{category: "lint/react/noUnsafe"},
		);
	},
);
