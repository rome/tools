class AbstractMembers {
    constructor();
    name(): string;
    get my_name();
    set my_name(name);
    #private_name();
}
abstract class AbstractMembers {
    abstract constructor() { }
    abstract display(): void { console.log(this.name); }
    abstract get my_name() { return this.name; }
    abstract set my_name(name) { this.name = name; }
    abstract #private_name() { }
}
