// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error my-change detail.value is a string
export const BadKebab = <fx-events onmy-change={(e) => e.detail.value.toFixed()} />;

// @ts-expect-error MyChange detail has checked, not value
export const BadPascal = <fx-events onMyChange={(e) => e.detail.value} />;
