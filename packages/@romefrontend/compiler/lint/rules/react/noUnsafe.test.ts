import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romefrontend/string-utils";

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
				filename: "file.tsx",
				category: "lint/react/noUnsafe",
			},
		);
	},
);
