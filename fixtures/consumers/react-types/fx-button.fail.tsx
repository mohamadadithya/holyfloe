// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error variant is a closed union
export const BadVariant = <fx-button variant="nope" />;

// @ts-expect-error fx-press detail is unknown
export const BadDetail = <fx-button onfx-press={(e) => e.detail.toUpperCase()} />;
