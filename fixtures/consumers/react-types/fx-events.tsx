// Must compile with the React 19 JSX augmentation from expected/react-types (included via tsconfig).
export const Ok = (
  <fx-events
    onmy-change={(e) => void e.detail.value.toUpperCase()}
    onmyChange={(e) => void e.detail.value.toFixed()}
    onMyChange={(e) => void (e.detail.checked === true)}
    onMYEVENT={(e) => void e.detail.count.toFixed()}
  />
);
