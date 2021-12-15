Seq(typeDef.interface.groups).forEach(group =>
    Seq(group.members).forEach((member, memberName) =>
      markdownDoc(
        member.doc,
        { typePath: typePath.concat(memberName.slice(1)),
         signatures: member.signatures }
      )
    )
)

const promiseFromCallback = fn =>
    new Promise((resolve, reject) =>
      fn((err, result) => {
        if (err) return reject(err);
        return resolve(result);
      })
    );

runtimeAgent.getProperties(
    objectId,
    false, // ownProperties
    false, // accessorPropertiesOnly
    false, // generatePreview
    (error, properties, internalProperties) => {
      return 1
    },
);
