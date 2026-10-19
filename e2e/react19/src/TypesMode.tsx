import { log } from '../../shared/log.ts';

/**
 * `types` mode (React 19 only): plain tags, no wrapper. React 19 attaches `on` + the exact DOM
 * event name as a listener and sets props the element has as properties. Only the runtime is
 * tested here; the JSX typing of fixtures/out/react-types is checked by `just typecheck-react`.
 */
export function TypesMode() {
  return (
    <>
      <fx-events
        id="native-events"
        onmy-change={(e: CustomEvent<{ value: string }>) => log(`native my-change:${e.detail.value}`)}
        onmyChange={(e: CustomEvent<{ value: number }>) => log(`native myChange:${e.detail.value}`)}
        onMyChange={(e: CustomEvent<{ checked: boolean }>) => log(`native MyChange:${e.detail.checked}`)}
        onMYEVENT={(e: CustomEvent<{ count: number }>) => log(`native MYEVENT:${e.detail.count}`)}
      />
      <fx-badge id="native-badge" count={5} tone="warning" />
      {/* OQ-008: a complex prop on an element that is not defined yet (#define-lazy). */}
      <fx-lazy id="native-lazy" items={['p', 'q']} label="Native" />
    </>
  );
}

// Loose JSX typing for the tags above, so editors don't flag this file.
declare module 'react' {
  namespace JSX {
    interface IntrinsicElements {
      'fx-events': Record<string, unknown>;
      'fx-badge': Record<string, unknown>;
      'fx-lazy': Record<string, unknown>;
    }
  }
}
