type OptionsFlags<Type> = {
  readonly [Property in keyof Type]: boolean;
};
