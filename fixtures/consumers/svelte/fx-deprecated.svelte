<!-- Must compile: a deprecated component and prop still type-check. -->
<script lang="ts">
  import { FxDeprecated } from '../../out/svelte/fx-deprecated/index.js';
</script>

<FxDeprecated tone="error" kind="x" />
