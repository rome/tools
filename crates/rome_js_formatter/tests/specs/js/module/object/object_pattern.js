// Break over multiple lines if it contains a sub pattern
const {b: {test}} = test;

// non-shorthand and whole statement exceeds line width
const { id, isStatic: isStatic, method, methodId, g } = privateNamesMap.get(name);

// default and whole statement exceeds line width
const { id, isStatic = true, method, methodId, gee } = privateNamesMap.get(name);

// sub pattern
const { id, isStatic: {sub}, method, methodId, gee } = privateNamesMap;


const { id, //middle comment
  isStatic, method, methodId, gee } = privateNamesMap;

try {
  // you should not break
} catch ({ data: { message }}) {

}