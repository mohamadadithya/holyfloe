// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error focused (from the mixin) is a boolean
export const BadFocused = <fx-mixin focused="yes" />;
