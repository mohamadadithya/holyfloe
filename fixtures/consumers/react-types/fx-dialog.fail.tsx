// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error open-change detail.open is a boolean
export const BadDetail = <fx-dialog onopen-change={(e) => e.detail.open.toUpperCase()} />;
