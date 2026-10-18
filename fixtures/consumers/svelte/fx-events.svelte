<!-- Must compile: every casing through its base name, with typed detail. -->
<script lang="ts">
  import { FxEvents } from '../../out/svelte/fx-events/index.js';
</script>

<FxEvents
  onMyChange={(e) => e.detail.value.toUpperCase()}
  onMyChangeCamel={(e) => e.detail.value.toFixed()}
  onMyChangePascal={(e) => e.detail.checked === true}
  onMYEVENT={(e) => e.detail.count.toFixed()}
/>
