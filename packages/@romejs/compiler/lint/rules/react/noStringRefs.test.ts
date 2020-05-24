import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"react no string refs",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				`
					class Hello extends React.Component {
						componentDidMount() {
							const component = this.refs.hello;
						}

						render() {
							return <div>Hello {this.props.name}</div>;
						}
					}
				`,
				`
					class Hello extends React.Component {
						render() {
							return <div ref="hello">Hello {this.props.name}</div>;
						}
					}
				`,
				`
					class Hello extends React.Component {
						render() {
							return <div ref={\`hello\`}>Hello {this.props.name}</div>;
						}
					}
				`,
				`
					class Hello extends React.Component {
						render() {
							return <div ref={'hello'}>Hello {this.props.name}</div>;
						}
					}
				`,
				`
					class Hello extends React.Component {
					  render() {
					    return <div ref={\`hello\${index}\`}>Hello {this.props.name}</div>;
					  }
					}
				`,
				`
					class Hello extends React.Component {
						componentDidMount() {
							const component = this.refs.hello;
						}

						render() {
							return <div ref="hello">Hello {this.props.name}</div>;
						}
					}
				`,
				// VALID
				`
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
			{category: "lint/react/noStringRefs"},
		);
	},
);
