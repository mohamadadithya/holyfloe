<!-- Server-rendered, then hydrated. Rendering every fixture also proves each element module imports
     in Node. Ids match e2e/nuxt/app/app.vue; tests/ssr.spec.ts runs on both. -->
<script lang="ts">
  import { FxBadge } from '../../../../fixtures/out/svelte/fx-badge/index.js';
  import { FxButton } from '../../../../fixtures/out/svelte/fx-button/index.js';
  import { FxCard } from '../../../../fixtures/out/svelte/fx-card/index.js';
  import { FxDeprecated } from '../../../../fixtures/out/svelte/fx-deprecated/index.js';
  import { FxDerived } from '../../../../fixtures/out/svelte/fx-derived/index.js';
  import { FxDialog } from '../../../../fixtures/out/svelte/fx-dialog/index.js';
  import { FxEvents } from '../../../../fixtures/out/svelte/fx-events/index.js';
  import { FxInput } from '../../../../fixtures/out/svelte/fx-input/index.js';
  import { FxList } from '../../../../fixtures/out/svelte/fx-list/index.js';
  import { FxMixin } from '../../../../fixtures/out/svelte/fx-mixin/index.js';
  import { FxSelect } from '../../../../fixtures/out/svelte/fx-select/index.js';

  let presses = $state(0);
  let text = $state('start');
  let open = $state(true);
  let choice = $state('b');
</script>

<main>
  <FxButton id="ssr-btn" variant="danger" disabled>Danger</FxButton>
  <FxButton id="client-btn" onFxPress={() => presses++}>Press</FxButton>
  <output id="presses">{presses}</output>
  <FxBadge id="ssr-badge" count={3} tone="success" label="3 unread" for="inbox">new</FxBadge>
  <FxList id="ssr-list" items={[{ id: 'a', label: 'Alpha' }]} options={{ dense: true }} />
  <FxCard id="ssr-card" elevated style="--fx-card-padding: 3px">
    {#snippet header()}<h2>Head</h2>{/snippet}
    <p>Body</p>
  </FxCard>
  <FxDerived id="ssr-derived" label="L" size="lg" />
  <FxDeprecated id="ssr-dep" tone="error" />
  <FxEvents id="ssr-events" />
  <FxInput id="ssr-input" bind:value={text} name="q" />
  <output id="ssr-input-state">{text}</output>
  <FxDialog id="ssr-dialog" bind:open>Dialog</FxDialog>
  <output id="ssr-dialog-state">{open}</output>
  <FxSelect id="ssr-select" bind:value={choice} options={['a', 'b']} />
  <FxMixin id="ssr-mixin" ringColor="red" focused>Mixed</FxMixin>
</main>
