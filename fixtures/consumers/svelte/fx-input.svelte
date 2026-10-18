<!-- Must compile: bind:value and the change event. -->
<script lang="ts">
  import { FxInput } from '../../out/svelte/fx-input/index.js';

  let text = $state('');
</script>

<FxInput bind:value={text} name="q" disabled={false} onChange={(e) => e.currentTarget.value.toUpperCase()} />
