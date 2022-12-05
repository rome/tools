// invalid

something.forEach((Element, index) => {
    return <div key={index}>foo</div>;
});

something.forEach((Element, index) => <div key={index}>foo</div>);

something.forEach((Element, index) => {
    return <List>
            <div key={index}>foo</div>
    </List>;
});

something.forEach((Element, index) => {
    return <List
        ><div key={index}>foo</div>
    </List>;
});

something.forEach((Element, index) => {
    return <List>
        "a b\
        cd"
        <div key={index}>foo</div>
    </List>;
});

something.forEach((Element, index) => {
    return <List>
        "a b\
        cd"<div key={index}>foo</div>
    </List>;
});