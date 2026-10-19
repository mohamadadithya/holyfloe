<!-- Must compile: complex, primitive and boolean props. -->
<script lang="ts">
  import { FxLazy } from '../../out/svelte/fx-lazy/index.js';
</script>

<FxLazy items={['a', 'b']} label="Fruit" active />
