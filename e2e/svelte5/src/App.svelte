<!-- Renders every fixture through the wrappers `just gen-fixtures` wrote to fixtures/out/svelte.
     Element ids match e2e/vue3/src/App.vue; tests/wrappers.spec.ts runs on both. -->
<script lang="ts">
  import { defineFxLazy } from '@holyfloe/fixtures/fx-lazy/fx-lazy.js';
  import { FxBadge } from '../../../fixtures/out/svelte/fx-badge/index.js';
  import { FxButton } from '../../../fixtures/out/svelte/fx-button/index.js';
  import { FxCard } from '../../../fixtures/out/svelte/fx-card/index.js';
  import { FxDeprecated } from '../../../fixtures/out/svelte/fx-deprecated/index.js';
  import { FxDerived } from '../../../fixtures/out/svelte/fx-derived/index.js';
  import { FxDialog } from '../../../fixtures/out/svelte/fx-dialog/index.js';
  import { FxEvents } from '../../../fixtures/out/svelte/fx-events/index.js';
  import { FxInput } from '../../../fixtures/out/svelte/fx-input/index.js';
  import { FxLazy } from '../../../fixtures/out/svelte/fx-lazy/index.js';
  import { FxList } from '../../../fixtures/out/svelte/fx-list/index.js';
  import { FxMixin } from '../../../fixtures/out/svelte/fx-mixin/index.js';
  import { FxSelect } from '../../../fixtures/out/svelte/fx-select/index.js';
  import { log } from '../../shared/log.ts';

  let renders = $state(0);
  let disabled = $state(true);
  let variant: 'danger' | undefined = $state('danger');
  let count = $state(3);
  let text = $state('start');
  let open = $state(false);
  let choice = $state('b');
  let button: HTMLElementTagNameMap['fx-button'] | undefined = $state();
  let card: HTMLElementTagNameMap['fx-card'] | undefined = $state();
  let dialog: HTMLElementTagNameMap['fx-dialog'] | undefined = $state();
  let mixin: HTMLElementTagNameMap['fx-mixin'] | undefined = $state();
  $effect(() => {
    window.refs = { button: button ?? null, card: card ?? null };
  });
</script>

<button id="rerender" onclick={() => renders++}>rerender {renders}</button>
<button id="toggle" onclick={() => (disabled = !disabled)}>toggle disabled</button>
<button id="unset-variant" onclick={() => (variant = undefined)}>unset variant</button>
<button id="inc" onclick={() => count++}>increment count</button>
<button id="define-lazy" onclick={defineFxLazy}>define fx-lazy</button>
<button id="open-dialog" onclick={() => (open = true)}>open dialog</button>
<button id="show-dialog" onclick={() => dialog?.show()}>show() on the element</button>
<button id="toggle-focus" onclick={() => mixin?.toggleFocus()}>toggleFocus() on the element</button>

<FxButton id="btn" bind:element={button} {variant} {disabled}>Go</FxButton>
<FxButton id="btn2" onFxPress={(e) => log(`press:${renders}:${e.currentTarget.tagName}`)}>Click</FxButton>
<FxButton id="plain" class="extra" data-x="1" onclick={() => log('native click')}>Plain</FxButton>

<FxBadge id="badge" {count} tone="success" label="3 unread" for="inbox">new</FxBadge>

<FxList
  id="list"
  items={[{ id: 'a', label: 'Alpha' }, { id: 'b', label: 'Beta' }, { id: 'c', label: 'Gamma' }]}
  options={{ dense: true, maxItems: 2 }}
  renderLabel={(item) => item.label.toUpperCase()}
/>

<FxEvents
  id="events"
  onMyChange={(e) => log(`my-change:${e.detail.value}`)}
  onMyChangeCamel={(e) => log(`myChange:${e.detail.value}`)}
  onMyChangePascal={(e) => log(`MyChange:${e.detail.checked}`)}
  onMYEVENT={(e) => log(`MYEVENT:${e.detail.count}`)}
/>

<FxCard id="card" bind:element={card} elevated style="--fx-card-padding: 3px; color: red">
  {#snippet header()}<h2>Head</h2>{/snippet}
  <p>Body</p>
  {#snippet footer()}<span>Foot</span>{/snippet}
</FxCard>
<FxCard id="card-plain"><p>Only a body</p></FxCard>

<FxDerived id="derived" label="L" size="lg" loading onFxReady={(e) => log(`ready:${e.currentTarget.tagName}`)}>kid</FxDerived>

<FxDeprecated id="dep" tone="error" kind="x" />

<FxInput id="input" bind:value={text} name="q" />
<output id="input-state">{text}</output>

<FxDialog id="dialog" bind:element={dialog} bind:open>
  Dialog body
  {#snippet footer()}<button>OK</button>{/snippet}
</FxDialog>
<output id="dialog-state">{open}</output>

<FxSelect id="select" bind:value={choice} options={['a', 'b', 'c']} />
<output id="select-state">{choice}</output>

<FxMixin id="mixin" bind:element={mixin} prefix="> " focused ringColor="red" onFocusChange={(e) => log(`focus:${e.currentTarget.focused}`)}>
  Mixed
</FxMixin>

<FxLazy id="lazy" items={['x', 'y']} label="Lazy" active />
