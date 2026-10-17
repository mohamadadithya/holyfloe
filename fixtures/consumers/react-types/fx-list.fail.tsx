// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error items need id and label strings
export const BadItems = <fx-list items={[{ id: 1 }]} />;
