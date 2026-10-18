// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error items are strings
export const BadItems = <fx-lazy items={[1]} />;
