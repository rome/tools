app.get("/", (req, res): void => {
    res.send("Hello World!");
})


export class Thing implements OtherThing {
    do: (type: Type) => Provider<Prop> = memoize(
        (type: ObjectType): Provider<Opts> => {}
    );
}