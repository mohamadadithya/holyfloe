// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error count is a number
export const BadCount = <fx-badge count="3" />;

