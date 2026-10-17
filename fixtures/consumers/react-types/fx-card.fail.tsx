// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error CSS custom properties are strings
export const BadVar = <fx-card style={{ '--fx-card-radius': 2 }} />;
