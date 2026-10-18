// Must compile: correct props, events, ref and children.
import { FxCard } from '../../out/react/fx-card/index.js';

export function Consumer() {
  return (
    <FxCard elevated style={{ '--fx-card-padding': '4px', color: 'red' }}>
      <h2 slot="header">Title</h2>
      Body
      <p slot="footer">Footer</p>
    </FxCard>
  );
}
