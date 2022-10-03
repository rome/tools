// invalid
React.createElement('button');
React.createElement('button', {
    "type": "bar"
});
React.createElement('button', {
    "style": "background: red"
});
React.createElement('button', {});

// valid
React.createElement('button', {
    "type": foo
});
