<>
    {/* invalid */}
    <html />
    <html></html>
    <html lang=""></html>
    <html lang={""}></html>
    <html lang={``}></html>
    <html lang={true}></html>
    <html lang={false}></html>
    <html lang={undefined}></html>
    <html lang={null}></html>
    <html {...props} lang=""></html>
    {/* valid */}
    <html lang="en"></html>
    <html lang={"en"}></html>
    <html lang={lang}></html>
    <html {...props}></html>
    <html lang="" {...props}></html>
</>