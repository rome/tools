// invalid
[0, [12]].flat().map(Number)
// valid
[0, [12], [[16]]].flat(1).map(Number)