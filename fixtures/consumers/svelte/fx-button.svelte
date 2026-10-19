<!-- Must compile: props, event, default slot and bind:element. -->
<script lang="ts">
  import { FxButton } from '../../out/svelte/fx-button/index.js';

  let element: HTMLElementTagNameMap['fx-button'] | undefined = $state();
</script>

<FxButton bind:element variant="danger" disabled={false} onFxPress={(e) => e.currentTarget.variant.toUpperCase()}>
  Go
</FxButton>
<FxButton onclick={() => element?.focus()}>Click</FxButton>
