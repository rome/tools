import x = require("./test");
namespace a.b {}
import y = a;
import z = a.b;
export import n = a;
