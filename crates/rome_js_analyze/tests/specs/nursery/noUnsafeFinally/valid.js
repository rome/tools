{ var foo = function() { try { return 1; } catch(err) { return 2; } finally { console.log('hola!') } } }
{ var foo = function() { try { return 1 } catch(err) { return 2 } finally { console.log('hola!') } } }
{ var foo = function() { try { return 1 } catch(err) { return 2 } finally { function a(x) { return x } } } }
{ var foo = function() { try { return 1 } catch(err) { return 2 } finally { var a = function(x) { if(!x) { throw new Error() } } } } }
{ var foo = function() { try { return 1 } catch(err) { return 2 } finally { var a = function(x) { while(true) { if(x) { break } else { continue } } } } } }
{ var foo = function() { try { return 1 } catch(err) { return 2 } finally { var a = function(x) { label: while(true) { if(x) { break label; } else { continue } } } } } }
{ var foo = function() { try {} finally { while (true) break; } } }
{ var foo = function() { try {} finally { while (true) continue; } } }
{ var foo = function() { try {} finally { switch (true) { case true: break; } } } }
{ var foo = function() { try {} finally { do { break; } while (true) } } }
{ var foo = function() { try { return 1; } catch(err) { return 2; } finally { var bar = () => { throw new Error(); }; } }; }
{ var foo = function() { try { return 1; } catch(err) { return 2 } finally { (x) => x } } }
{ var foo = function() { try { return 1; } finally { class bar { constructor() {} static ehm() { return 'Hola!'; } } } }; }