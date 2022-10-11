// invalid
[0, [12]].map(Number).flat()
[0, [12], [[16]]].map(Number).flat(1)
// valid
[0, [12], [[16]]].map(Number).flat(2)
