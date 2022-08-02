let inlinable = "value1";
let notInlinable = "value2";

if (inlinable) {
    notInlinable = inlinable;
}

statement(notInlinable);
