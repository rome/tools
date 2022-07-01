function JsTryFinallyStatement1() {
    try {
        test();
    } catch (err) {
        test();
    } finally {
        test();
        return;
    }

    afterFinallyReturn();
}
