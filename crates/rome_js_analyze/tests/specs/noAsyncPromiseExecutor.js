// valid
new Promise((resolve, reject) => {})
new Promise((resolve, reject) => {}, async function unrelated() {})
new Foo(async (resolve, reject) => {})
new Foo((( (resolve, reject) => {} )))
// invalid
new Promise(async function foo(resolve, reject) {})
new Promise(async (resolve, reject) => {})
new Promise(((((async () => {})))))