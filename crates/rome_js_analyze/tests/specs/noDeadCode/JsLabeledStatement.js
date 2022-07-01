function JsLabeledStatement1() {
    label: while (true) {
        while (true) {
            if (true) {
                continue label;
            } else {
                break label;
            }
        }

        afterLabelJump();
    }
}
