// invalid
let a = <html lang="lorem" />;
let a = <html lang="en-babab" />;

// valid
let a = <Html lang="en-babab" />;
let a = <html lang="en-US"></html>;
let a = <html lang="en"></html>;
let a = <html lang={lang}></html>;
