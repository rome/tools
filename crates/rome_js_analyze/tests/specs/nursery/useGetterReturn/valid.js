var foo = {
    get bar(){
        return true;
    }
};

var foo = {
    get bar(){
        throw new Error("no value");
    }
};

var foo = {
    get bar(){
        if(baz) {
            if (foo) {
               return true;
            } else {
               return true;
            }
        } else {
            return true;
        }
    }
};

class Foo {
    get bar(){
        return true;
    }
}

class Foo {
    get bar(){
        if (baz) {
            let x = 0;
            while (cond) {
                x++;
            }
            return x;
        } else {
            return 0;
        }
    }
}

class Foo {
    get bar(){
        try {
            return foo();
        } catch {} finally {
            return 0;
        }
    }
}

class Foo {
    get bar(){
        try {
            return foo();
        } catch {
            return 0;
        } finally {}
    }
}

class Foo {
    get bar(){
        try {
            foo();
        } catch {} finally {
            return 0;
        }
    }
}

class Foo {
    get bar(){
        try {
            foo();
        } finally {
            return 0;
        }
    }
}

class Foo {
    get(){
        switch (this.n) {
            case 1:
            case 2:
                return 2;
            default:
                return 0;
        }
    }
}

Object.defineProperty(foo, "bar", {
    get: function () {
        return true;
    }
});

Object.defineProperty(foo, "bar", {
    get: function () {
        ~function (){ return true; }();
        return true;
    }
});

Object.defineProperties(foo, {
    bar: {
        get: function () {
            return true;
        }
    }
});

Object.defineProperties(foo, {
    bar: {
        get: function () {
            ~function (){
                return true;
            }();
            return true;
        }
    }
});

Reflect.defineProperty(foo, "bar", {
    get: function () {
        return true;
    }
});

Reflect.defineProperty(foo, "bar", {
    get: function () {
        ~function (){
            return true;
        }();
        return true;
    }
})

Object.create(foo, {
    bar: {
        get() {
            return true;
        }
    }
});

Object.create(foo, { bar: {
    get: function () {
        return true;}
    }
});

Object.create(foo, { bar: {
    get: () => {
        return true;}
    }
});

// not getter.
var get = function(){};

var get = function(){ return true; };

var foo = { bar(){} };

var foo = { bar(){ return true; } };

var foo = { bar: function(){} };

var foo = { bar: function(){return;} };

var foo = { bar: function(){return true;} };

var foo = { get: function () {} }

var foo = { get: () => {}};

class C { get; foo() {} }

foo.defineProperty(null, { get() {} });

foo.defineProperties(null, { bar: { get() {} } });

foo.create(null, { bar: { get() {} } })
