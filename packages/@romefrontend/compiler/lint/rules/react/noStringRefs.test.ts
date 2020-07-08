import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romefrontend/string-utils";

test(
	"react no string refs",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						class Hello extends React.Component {
							componentDidMount() {
								const component = this.refs.hello;
							}

							render() {
								return <div>Hello {this.props.name}</div>;
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							render() {
								return <div ref="hello">Hello {this.props.name}</div>;
							}
						}
					`,
					// dedent currently passes backslashes through raw
					dedent`
						class Hello extends React.Component {
							render() {
								return <div ref={${"`"}hello${"`"}}>Hello {this.props.name}</div>;
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							render() {
								return <div ref={'hello'}>Hello {this.props.name}</div>;
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							render() {
								return <div ref={${"`"}hello${"$"}{index}${"`"}}>Hello {this.props.name}</div>;
							}
						}
					`,
					dedent`
						class Hello extends React.Component {
							componentDidMount() {
								const component = this.refs.hello;
							}

							render() {
								return <div ref="hello">Hello {this.props.name}</div>;
							}
						}
					`,
				],
				valid: [
					dedent`
						class Hello extends React.Component {
							componentDidMount() {
								const component = this.hello;
							}

							render() {
								return <div ref={c => this.hello = c}>Hello {this.props.name}</div>;
							}
						}
					`,
				],
			},
			{category: "lint/react/noStringRefs"},
		);
	},
);
