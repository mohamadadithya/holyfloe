<!-- Must compile: complex props typed from an imported type. -->
<script lang="ts">
  import type { ListItem } from '@holyfloe/fixtures/fx-list/types.js';
  import { FxList } from '../../out/svelte/fx-list/index.js';

  const items: ListItem[] = [{ id: 'a', label: 'Apple' }];
</script>

<FxList {items} options={{ dense: true, maxItems: 5 }} renderLabel={(item) => item.label.toUpperCase()} />
