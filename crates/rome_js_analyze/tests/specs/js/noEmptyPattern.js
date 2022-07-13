// valid
var {a = {}} = foo;
var {a, b = {}} = foo;
var {a = []} = foo;
function foo({a = {}}) {}
function foo({a = []}) {}
var [a] = foo;
// invalid
var {} = foo;
var [] = foo;
var {a: {}} = foo;
var {a, b: {}} = foo;
var {a: []} = foo;
function foo({}) {};
function foo([]) {};
function foo({a: {}}) {};
function foo({a: []}) {};