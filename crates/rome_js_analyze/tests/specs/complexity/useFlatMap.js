// invalid
[0, [12]].map(Number).flat();
[0, [12], [[16]]].map(Number).flat(1);
[0, [12], [[16]]].map((element) => {}).flat(1);
[0, [12], [[16]]].map((element, index) => {}).flat(1);
[0, [12], [[16]]].map((element, index, array) => {}).flat(1);
// valid
[0, [12], [[16]]].map(Number).flat(2);
array.flat().map(Number);
array.map(Number).flat(customFn);
MyObject.map(one, two, tree).flat();
MyObject.map().flat(1, "test")