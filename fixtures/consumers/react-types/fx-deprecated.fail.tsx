// Each marked line must fail to type-check (React 19 JSX augmentation).

// @ts-expect-error tone is a closed union
export const BadTone = <fx-deprecated tone="warn" />;
