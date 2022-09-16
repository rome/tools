// invalid
React.createElement('button');
React.createElement('button', {
    "type": "bar"
});
React.createElement('button', {
    "style": "background: red"
});

// valid
React.createElement('button', {
    "type": foo
});
