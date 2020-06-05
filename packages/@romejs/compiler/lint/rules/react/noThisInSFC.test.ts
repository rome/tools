import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"react no this in stateless functional component",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"const Foo = () => <div>{this.props.bar</div>",
					`
						const Foo = () => {
							return (
								<div>{this.props.bar}</div>
							);
						}
					`,
					`
						const Foo = () => {
							const { bar } = this.props;
							return (
								<div>{ bar }</div>
							);
						}
					`,
					`
						function Foo(props) {
							return (
								<div>{this.props.bar}</div>
							);
						}
					`,
					`
						function Foo(props) {
							const { bar } = this.props;
							return (
								<div>
									{ bar }
								</div>
							);
						}
					`,
					`
						function Foo(props, context) {
							return (
								<div>
									{this.context.foo ? this.props.bar : ''}
								</div>
							);
						}				
					`,
					`
						function Foo(props, context) {
							const { foo } = this.context;
							const { bar } = this.props;
							return (
								<div>
									{foo ? bar : ''}
								</div>
							);
						}
					`,
					`
						function Foo(props) {
							if (this.state.loading) {
								return <Loader />;
							}
							return (
								<div>
									{this.props.bar}
								</div>
							);
						}
					`,
					`
						function Foo(props) {
							const { loading } = this.state;
							const { bar } = this.props;
							if (loading) {
								return <Loader />;
							}
							return (
								<div>
									{bar}
								</div>
							);
						}				
					`,
					`
						React.memo(
							function Foo(props) {
								return (
									<div>{this.props.bar}</div>
								);
							}
						)
					`,
					`
						React.forwardRef((props, ref) => (
							<div>
								{this.props.bar}
							</div>
						));
					`,
					`
						const Foo = React.forwardRef((props, ref) => (
							<div>
								{this.props.bar}
							</div>
						));
					`,
					`
						const Foo = React.memo((props, ref) => (
								<div>
									{this.props.bar}
								</div>
							)
						)
					`,
				],
				valid: [
					"const Foo = (bar) => <div>{bar}</div>",
					`
						function Foo(props) {
							return (
								<div>{props.bar}</div>
							);
						}
					`,
					`
						function Foo(props) {
							const { bar } = props;
							return (
								<div>{bar}</div>
							);
						}
					`,
					`
						function Foo({ bar }) {
							return (
								<div>{bar}</div>
							);
						}
					`,
					`
						function Foo(props, context) {
							return (
								<div>
									{context.foo ? props.bar : ''}
								</div>
							);
					}
					`,
					`
						function Foo(props, context) {
							const { foo } = context;
							const { bar } = props;
							return (
								<div>
									{foo ? bar : ''}
								</div>
							);
						}
					`,
					`
						function Foo({ bar }, { foo }) {
							return (
								<div>
									{foo ? bar : ''}
								</div>
							);
						}
					`,
					`
						class MyComponent extends React.Component {
							foo() {
									return <div>some jsx</div>
							}

							render() {
								return "content"
							}
						}
					`,
					`
					React.memo(
						function Foo(props) {
							return (
								<div>{props.bar}</div>
							);
						}
					)
				`,
					`
					React.forwardRef((props, ref) => (
						<div>
							{props.bar}
						</div>
					));
				`,
					`
					const Foo = React.forwardRef((props, ref) => (
						<div>
							{props.bar}
						</div>
					));
				`,
					`
					const Foo = React.memo((props, ref) => (
							<div>
								{props.bar}
							</div>
						)
					)
				`,
				],
			},
			{category: "lint/react/noThisInSFC"},
		);
	},
);
