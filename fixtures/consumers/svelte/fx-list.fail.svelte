<!-- Each marked line must fail to type-check (props objects: svelte-check has no expect-error for markup). -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import { FxList } from '../../out/svelte/fx-list/index.js';

  type Props = ComponentProps<typeof FxList>;
  // @ts-expect-error items need id and label strings
  export const badItems: Props = { items: [{ id: 1 }] };
  // @ts-expect-error dense is boolean
  export const badOptions: Props = { options: { dense: 'yes' } };
  // @ts-expect-error renderLabel returns a string
  export const badRenderer: Props = { renderLabel: () => 42 };
</script>
