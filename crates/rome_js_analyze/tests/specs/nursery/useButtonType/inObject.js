// invalid
React.createElement('button');
React.createElement('button', {
    "type": "bar"
});

// valid
React.createElement('button', {
    "type": foo
});
