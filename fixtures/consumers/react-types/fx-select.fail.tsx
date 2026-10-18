// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error options are strings
export const BadOptions = <fx-select options={[1]} />;
