<!-- Must compile: inherited props and event. -->
<script lang="ts">
  import { FxDerived } from '../../out/svelte/fx-derived/index.js';
</script>

<FxDerived label="L" size="lg" loading onFxReady={(e) => e.currentTarget.size}>kid</FxDerived>
