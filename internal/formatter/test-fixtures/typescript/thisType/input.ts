function bound(this: ToBind) {};
type Buond = (this: ToBind) => void;

class ToBind {
    private bound: Buond;
    constructor() {
        this.bound = bound.bind(this);
        this.bound();
    }
}
