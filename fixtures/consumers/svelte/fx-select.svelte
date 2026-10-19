<!-- Must compile: bind:value with the value from detail.value. -->
<script lang="ts">
  import { FxSelect } from '../../out/svelte/fx-select/index.js';

  let choice = $state('a');
</script>

<FxSelect bind:value={choice} options={['a', 'b']} onFxChange={(e) => e.detail.value.toUpperCase()} />
