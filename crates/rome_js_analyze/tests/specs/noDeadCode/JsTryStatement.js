function JsTryStatement1() {
    try {
        test();
        return;
    } catch (err) {
        test();
        return;
    }

    afterTryCatchReturn();
}
