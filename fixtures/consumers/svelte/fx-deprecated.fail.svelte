<!-- Each marked line must fail to type-check (props objects: svelte-check has no expect-error for markup). -->
<script lang="ts">
  import type { ComponentProps } from 'svelte';
  import { FxDeprecated } from '../../out/svelte/fx-deprecated/index.js';

  // @ts-expect-error tone is a union of strings
  export const badTone: ComponentProps<typeof FxDeprecated> = { tone: 'loud' };
</script>
