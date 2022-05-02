let a = "test";
type B = typeof a;
class C {
    #a = 'a';
    constructor() {
        const a: typeof this.#a = ''; 
        const b: typeof this.#a = 1; 
    }
}
const c = new C();
const a: typeof c.#a = '';
