// see https://github.com/rome/tools/issues/3697

const romeKiller = () => {
    const fn = (callback) => {
      callback(fn);
    };
  };