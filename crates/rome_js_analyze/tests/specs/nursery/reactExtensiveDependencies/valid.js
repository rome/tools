// No captures
function MyComponent1() {
    useEffect(() => {
    });
}

// All captures in the dependency list
function MyComponent2() {
    const local = 1;
    useEffect(() => {
        console.log(local);
    }, [local]);
}

// capturing declarations
import { F } from 'something';

function doSomething() { }
class A {}

function MyComponent3() {
    useEffect(() => {
        doSomething();
        console.log(new A ());
        console.log(F);
    }, []);
}
