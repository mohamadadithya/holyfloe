// Must compile with the React 19 JSX augmentation from out/react-types (included via tsconfig).
export const Ok = <fx-input value="a" name="q" disabled onchange={(e) => void e.target.value.toUpperCase()} />;
