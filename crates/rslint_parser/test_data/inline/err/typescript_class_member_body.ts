class AbstractMembers {
    name(): string;
}
abstract class AbstractMembers {
    abstract display(): void { console.log(this.name); }
    abstract get my_name() { return this.name; }
    abstract set my_name(name) { this.name = name; }
}
