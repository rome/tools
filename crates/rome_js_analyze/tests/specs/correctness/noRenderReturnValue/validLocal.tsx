function render() { }
const ReactDOM = { render };

const a = ReactDOM.render(<div />, document.body);
const foo = bar && ReactDOM.render(<div />, document.body);
const foo = bar ? ReactDOM.render(<div />, document.body) : null
const foo = () => ReactDOM.render(<div />, document.body);
const foo = {
    react: ReactDOM.render(<div />, document.body)
};
let lorem;
lorem = ReactDOM.render(<div />, document.body);
function render1() {
    return ReactDOM.render(<div />, document.body)
}

const a = render(<div />, document.body);
const foo = bar && render(<div />, document.body);
const foo = bar ? render(<div />, document.body) : null
const foo = () => render(<div />, document.body);
const foo = {
    react: render(<div />, document.body)
};
let lorem;
lorem = render(<div />, document.body);
function render1() {
    return render(<div />, document.body)
}
