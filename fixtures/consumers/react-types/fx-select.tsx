// Must compile with the React 19 JSX augmentation from out/react-types (included via tsconfig).
export const Ok = <fx-select value="a" options={['a']} onfx-change={(e) => void e.detail.value.toUpperCase()} />;
