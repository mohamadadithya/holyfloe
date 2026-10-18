// Must compile: correct props, events, ref and children.
import type { FxEvents as FxEventsElement } from '@holyfloe/fixtures/fx-events/fx-events.js';
import { FxEvents } from '../../out/react/fx-events/index.js';

export function Consumer() {
  return (
    <FxEvents
      onMyChange={(e) => {
        const value: string = e.detail.value;
        const el: FxEventsElement = e.currentTarget;
        void value;
        void el;
      }}
      onMyChangeCamel={(e) => {
        const value: number = e.detail.value;
        void value;
      }}
      onMyChangePascal={(e) => {
        const checked: boolean = e.detail.checked;
        void checked;
      }}
      onMYEVENT={(e) => {
        const count: number = e.detail.count;
        void count;
      }}
    />
  );
}
