<!-- Each marked line must fail to type-check (raw tags, as attribute objects). -->
<script lang="ts">
  import type { SvelteHTMLElements } from 'svelte/elements';

  // @ts-expect-error variant is a union of strings
  export const badButton: SvelteHTMLElements['fx-button'] = { variant: 'huge' };
  // @ts-expect-error badge-count is a number
  export const badBadge: SvelteHTMLElements['fx-badge'] = { 'badge-count': '1' };
  // @ts-expect-error my-change detail.value is a string
  export const badEvents: SvelteHTMLElements['fx-events'] = { 'onmy-change': (e) => e.detail.value.toFixed() };
  // @ts-expect-error options are strings
  export const badSelect: SvelteHTMLElements['fx-select'] = { options: [1] };
</script>
