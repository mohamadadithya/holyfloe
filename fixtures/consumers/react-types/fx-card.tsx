// Must compile with the React 19 JSX augmentation from out/react-types (included via tsconfig).
export const Ok = (
  <fx-card elevated style={{ '--fx-card-radius': '2px', color: 'red' }}>
    <h2 slot="header">Title</h2>
  </fx-card>
);
