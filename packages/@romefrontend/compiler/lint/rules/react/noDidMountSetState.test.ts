import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romefrontend/string-utils";

test(
	"react no did mount set state",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						class Hello extends React.Component {
							componentDidMount() {
								this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							componentDidMount() {
								foo();
								this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends Component {
							componentDidMount() {
								this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends Component {
							componentDidMount() {
								foo();
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
							componentDidMount() {
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
							componentDidMount() {
								condition && this.setState({
									name: 'John'
								});
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							componentDidMount() {
								condition ? this.setState({
									name: 'John'
								}) : undefined;
							}
						}
					`,
				],
				filename: "file.tsx",
				category: "lint/react/noDidMountSetState",
			},
		);
	},
);
