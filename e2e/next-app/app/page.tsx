// A server component rendering the 'use client' wrappers that the e2e `test` script copies from
// fixtures/out/react to generated/. Rendering all 7 also proves every element module imports in
// Node without crashing. Only serializable props cross to the client; handlers live in Clicker.
import { FxBadge } from '../generated/fx-badge/index.js';
import { FxButton } from '../generated/fx-button/index.js';
import { FxCard } from '../generated/fx-card/index.js';
import { FxDeprecated } from '../generated/fx-deprecated/index.js';
import { FxDerived } from '../generated/fx-derived/index.js';
import { FxEvents } from '../generated/fx-events/index.js';
import { FxList } from '../generated/fx-list/index.js';
import { Clicker } from './Clicker.js';

export default function Page() {
  return (
    <main>
      <FxButton id="ssr-btn" variant="danger" disabled>Danger</FxButton>
      <FxBadge id="ssr-badge" count={3} tone="success" label="3 unread" for="inbox">new</FxBadge>
      <FxList id="ssr-list" items={[{ id: 'a', label: 'Alpha' }]} options={{ dense: true }} />
      <FxCard id="ssr-card" elevated style={{ '--fx-card-padding': '3px' }}>
        <h2 slot="header">Head</h2>
        <p>Body</p>
      </FxCard>
      <FxDerived id="ssr-derived" label="L" size="lg" />
      <FxDeprecated id="ssr-dep" tone="error" />
      <FxEvents id="ssr-events" />
      <Clicker />
    </main>
  );
}
