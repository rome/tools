import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romejs/string-utils";

test(
	"react sort comp",
	async (t) => {
		await testLint(
			t,
			{
				valid: [
					dedent`
            // Must validate a full class
            class Hello extends React.Component {
              displayName = ''
              propTypes = {}
              contextTypes = {}
              childContextTypes = {}
              mixins = []
              statics = {}
              getDefaultProps() {}
              getInitialState() {}
              getChildContext() {}
              componentWillMount() {}
              componentDidMount() {}
              componentWillReceiveProps() {}
              shouldComponentUpdate() {}
              componentWillUpdate() {}
              componentDidUpdate() {}
              componentWillUnmount() {}
              render() {
                return <div>Hello</div>;
              }
            };
          `,
					dedent`
            // Must validate a class with missing groups
            class Hello extends React.Component {
              render() {
                return <div>Hello</div>;
              }
            };
          `,
					dedent`
            // Must put a custom method in \'everything-else\'
            class Hello extends React.Component {
              onClick() {}
              render() {
                return <button onClick={this.onClick}>Hello</button>;
              }
            };
          `,
					dedent`
            // Must validate a full React class
            class Hello extends React.Component {
              displayName = \'\'
              propTypes = {}
              contextTypes = {}
              childContextTypes = {}
              mixins = []
              statics = {}
              getDefaultProps() {}
              getInitialState() {}
              getChildContext() {}
              UNSAFE_componentWillMount() {}
              componentDidMount() {}
              UNSAFE_componentWillReceiveProps() {}
              shouldComponentUpdate() {}
              UNSAFE_componentWillUpdate() {}
              getSnapshotBeforeUpdate() {}
              componentDidUpdate() {}
              componentDidCatch() {}
              componentWillUnmount() {}
              render() {
                return <div>Hello</div>;
              }
            };
          `,
					dedent`
						// Must validate React 16.3 lifecycle methods with the default parser
						class Hello extends React.Component {
							constructor() {}
							static getDerivedStateFromProps() {}
							UNSAFE_componentWillMount() {}
							componentDidMount() {}
							UNSAFE_componentWillReceiveProps() {}
							shouldComponentUpdate() {}
							UNSAFE_componentWillUpdate() {}
							getSnapshotBeforeUpdate() {}
							componentDidUpdate() {}
							componentDidCatch() {}
							componentWillUnmount() {}
							testInstanceMethod() {}
							render() { return (<div>Hello</div>); }
						}
					`,
					dedent`
						// Must validate a full React 16.3 ES6 class
						class Hello extends React.Component {
							static displayName = ''
							static propTypes = {}
							static defaultProps = {}
							constructor() {}
							state = {}
							static getDerivedStateFromProps = () => {}
							UNSAFE_componentWillMount = () => {}
							componentDidMount = () => {}
							UNSAFE_componentWillReceiveProps = () => {}
							shouldComponentUpdate = () => {}
							UNSAFE_componentWillUpdate = () => {}
							getSnapshotBeforeUpdate = () => {}
							componentDidUpdate = () => {}
							componentDidCatch = () => {}
							componentWillUnmount = () => {}
							testArrowMethod = () => {}
							testInstanceMethod() {}
							render = () => (<div>Hello</div>)
						}
					`,
					dedent`
						// Must allow us to use 'constructor' as a method name
						class Hello extends React.Component {
							constructor() {}
							displayName() {}
							render() {
								return <div>Hello</div>;
							}
						}
					`,
					dedent`
						// Must ignore stateless components
						function Hello(props) {
							return <div>Hello {props.name}</div>
						}
					`,
					dedent`
						// Must ignore stateless components (arrow function with explicit return)
						var Hello = props => (
							<div>Hello {props.name}</div>
						)
					`,
					dedent`
						// Non-react classes should be ignored, even in expressions
						return class Hello {
							render() {
								return <div>{this.props.text}</div>;
							}
							props: { text: string };
							constructor() {}
							state: Object = {};
						}
					`,
					dedent`
						// Non-react classes should be ignored, even in expressions
						return class {
							render() {
								return <div>{this.props.text}</div>;
							}
							props: { text: string };
							constructor() {}
							state: Object = {};
						}
					`,
					dedent`
						// static lifecycle methods can be grouped (with lifecycle)
						class Hello extends React.Component {
							constructor() {}
							static getDerivedStateFromProps() {}
						}
					`,
					dedent`
          class MyComponent extends React.Component {
            static propTypes;
            state = {};
            foo;
            render() {
              return null;
            }
          }
          `,
					dedent`
            class MyComponent extends React.Component {
              static getDerivedStateFromProps() {}
              static foo;
              render() {
                return null;
              }
            }
          `,
					dedent`
            class MyComponent extends React.Component {
              static getDerivedStateFromProps() {}
              static foo = 'some-str';
              render() {
                return null;
              }
            }
          `,
					dedent`
            class MyComponent extends React.Component {
              static getDerivedStateFromProps() {}
              foo = {};
              static bar = 0;
              render() {
                return null;
              }
            }
          `,
					dedent`
            class MyComponent extends React.Component {
              static getDerivedStateFromProps() {}
              static bar = 1;
              foo = {};
              render() {
                return null;
              }
            }
          `,
					dedent`
            class MyComponent extends React.Component {
              constructor() {
                super(props);
                this.state = {};
              }
              static foo = 1;
              bar;
              render() {
                return null;
              }
            }
          `,
				],
				invalid: [
					dedent`
						// Must force a lifecycle method to be placed before render
						class Hello extends React.Component {
							render() {
								return <div>Hello</div>;
							}
							displayName = 'Hello'
						};
					`,
					dedent`
						// Must force a custom method to be placed before render
						class Hello extends React.Component {
							render() {
								return <div>Hello</div>;
							}
							onClick() {}
						};
					`,
					dedent`
						// Must force a custom method to be placed before render, even in function
						var Hello = () => {
							return class Test extends React.Component {
								render () {
									return <div>Hello</div>;
								}
								onClick () {}
							}
						};
					`,
					dedent`
						// Type Annotations should not be at the top by default
						class Hello extends React.Component {
							props: { text: string };
							constructor() {}
							state: Object = {};
							render() {
								return <div>{this.props.text}</div>;
							}
						}
					`,
				],
			},
			{category: "lint/react/sortComp"},
		);
	},
);
