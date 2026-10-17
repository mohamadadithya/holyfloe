// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error count is a number
export const BadCount = <fx-badge count="3" />;

// @ts-expect-error overflow is a read-only getter, so it is not a prop
export const ReadOnly = <fx-badge overflow />;

