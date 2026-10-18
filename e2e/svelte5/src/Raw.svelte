<!-- Plain tags, no wrapper (OQ-004): Svelte's own property-vs-attribute choice on custom elements. -->
<script lang="ts">
  import { defineFxLazy } from '@holyfloe/fixtures/fx-lazy/fx-lazy.js';
</script>

<!-- No raw `bind:value` here: Svelte 5.57.2 rejects it on custom elements at compile time
     ("`bind:value` can only be used with `<input>`, `<textarea>`, `<select>`"). -->

<button id="define-lazy" onclick={defineFxLazy}>define fx-lazy</button>
<fx-lazy id="raw-lazy" items={['p', 'q']} label="Raw"></fx-lazy>
<fx-lazy id="raw-lazy-fn" items={() => 'fn'} label="Fn"></fx-lazy>
