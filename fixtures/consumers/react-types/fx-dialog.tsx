// Must compile with the React 19 JSX augmentation from out/react-types (included via tsconfig).
export const Ok = <fx-dialog open onopen-change={(e) => void (e.detail.open === true)} />;
