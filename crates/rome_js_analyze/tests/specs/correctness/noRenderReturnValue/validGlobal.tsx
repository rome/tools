ReactDOM.render(<div />, document.body);
function render1() {
    ReactDOM.render(<div />, document.body);
}
const render2 = () => {
    ReactDOM.render(<div />, document.body);
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
function render3() {
    return render(<div />, document.body)
}
