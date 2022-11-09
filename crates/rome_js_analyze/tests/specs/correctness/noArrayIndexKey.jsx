import { Children, cloneElement } from "react";

// invalid
something.forEach((Element, index) => {
    <Component key={index} >foo</Component>
});
something.forEach((element, index, array) => {
    <Component key={index} >foo</Component>
});
things.filter((thing, index) => {
    otherThings.push(<Hello key={index} >foo</Hello>);
});

something.forEach((Element, index) => {
    <Component key={index} />
});
something.forEach((element, index, array) => {
    <Component key={index} />
});
things.filter((thing, index) => {
    otherThings.push(<Hello key={index} />);
});
things.reduce((collection, thing, index) => (
    collection.concat(<Hello key={index} />)
), []);

React.Children.map(this.props.children, (child, index) => (
    React.cloneElement(child, { key: index })
))

React.Children.forEach(this.props.children, function (child, index) {
    return React.cloneElement(child, { key: index })
})


Children.map(this.props.children, (child, index) => (
    cloneElement(child, { key: index })
))

Children.forEach(this.props.children, function (child, index) {
    return cloneElement(child, { key: index })
})

Children.forEach(this.props.children, function (child, index) {
    const foo = cloneElement(child, { key: index })
    return foo;
})


things.map((thing, index) => (
    React.cloneElement(thing, { key: index })
));


// valid
something.forEach((element, index) => {
    <Component key={index + "something"} >foo</Component>
});
something.forEach((element, index) => {
    <Component key={index + "something"} />

});
