
useEffect(() => {

}, [a, b])

useMemo(() => {
    return {
        d, e
    }
}, [a, b])

useMemo(() => {

    } // some comment
    ,
    [a, b]
)

test.expect(t => {
    t.true(a)
})

test.expect(t => {
    t.true(a)
}, false)

test.something(t => {
    t.true()
}, context => {
    context.flush()
})