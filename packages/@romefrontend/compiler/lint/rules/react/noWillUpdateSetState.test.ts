import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romefrontend/string-utils";

test(
	"react no will update set state",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						class Hello extends React.Component {
							componentWillUpdate() {
								this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							componentWillUpdate() {
								foo();
								this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends Component {
							componentWillUpdate() {
								this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends Component {
							componentWillUpdate() {
								foo();
								this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							UNSAFE_componentWillUpdate() {
								this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends Component {
							UNSAFE_componentWillUpdate() {
								this.setState({
									name: 'John'
								});
							}
						}
					`,
				],
				valid: [
					dedent`
						class Hello extends React.Component {
							componentWillUpdate() {
								if (condition) {
									this.setState({
										name: 'John'
									});
								}
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							componentWillUpdate() {
								condition && this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							componentWillUpdate() {
								condition ? this.setState({
									name: 'John'
								}) : undefined;
							}
						}
					`,
				],
			},
			{category: "lint/react/noWillUpdateSetState"},
		);
	},
);
