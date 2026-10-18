// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error value is a string
export const BadValue = <fx-input value={1} />;
