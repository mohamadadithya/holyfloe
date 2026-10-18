<!-- Must compile: default and named slots as snippets, boolean prop. -->
<script lang="ts">
  import { FxCard } from '../../out/svelte/fx-card/index.js';
</script>

<FxCard elevated style="--fx-card-padding: 3px">
  {#snippet header()}<h2>Head</h2>{/snippet}
  <p>Body</p>
  {#snippet footer()}<span>Foot</span>{/snippet}
</FxCard>
