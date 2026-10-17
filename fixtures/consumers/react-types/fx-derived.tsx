// Must compile with the React 19 JSX augmentation from expected/react-types (included via tsconfig).
export const Ok = <fx-derived label="Hi" size="sm" loading onfx-ready={(e) => void e.currentTarget.size} />;
