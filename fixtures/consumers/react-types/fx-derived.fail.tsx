// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error inherited size is a closed union
export const BadSize = <fx-derived size="xl" />;
