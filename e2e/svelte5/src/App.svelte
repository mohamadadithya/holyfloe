<script lang="ts">
  import { FxButton } from '../../../fixtures/out/svelte/fx-button/index.js';
  import { log } from '../../shared/log.ts';

  let disabled = $state(true);
  let renders = $state(0);
  let variant: 'danger' | undefined = $state('danger');
  let element: HTMLElementTagNameMap['fx-button'] | undefined = $state();
  $effect(() => {
    window.refs = { button: element ?? null };
  });
</script>

<button id="toggle" onclick={() => (disabled = !disabled)}>toggle disabled</button>
<button id="rerender" onclick={() => renders++}>rerender {renders}</button>
<button id="unset-variant" onclick={() => (variant = undefined)}>unset variant</button>
<FxButton id="btn" bind:element {variant} {disabled}>Go</FxButton>
<FxButton id="btn2" onFxPress={(e) => log(`press:${renders}:${e.currentTarget.tagName}`)}>Click</FxButton>
<FxButton id="plain" class="extra" data-x="1" onclick={() => log('native click')}>Plain</FxButton>
