function functionDeclaration() {
    return 0;
}

const usingThis = function() {
    return this;
}

const generatorExpression = function* (this: Number) {
    return 0;
}

const named = function named (this: Number) {
    return 0;
}

const withThisParameter = function(this: Number) {
    return 0;
}

const g = function(a) {
    if (a != null) {
        return () => this;
    }
    return () => {};
}

export class Counter {
    static {
        // do nothings
    }

    constructor() {
        this.count = 0;
    }

    get count() {
        return this.count;
    }

    set count(val) {
        this.count = val;
    }

    increment() {
        this.count++;
    }

    get initial() {
        return 0;
    }

    initial_value() {
        return 0;
    }
}

export const DEFAULT_COUNTER = {
    get count() {
        return this.count;
    },

    set count(val) {
        this.count = val;
    },

    increment() {
        this.count++;
    },

    get initial() {
        return 0;
    },

    initial_value() {
        return 0;
    },
}

export default function() {
    return 0;
}
