<!-- Must compile: complex props typed from an imported type. -->
<script setup lang="ts">
import type { ListItem } from '@holyfloe/fixtures/fx-list/types.js';
import { FxList } from '../../out/vue/fx-list/index.js';

const items: ListItem[] = [{ id: 'a', label: 'Apple' }];
</script>

<template>
  <FxList :items="items" :options="{ dense: true, maxItems: 5 }" :render-label="(item) => item.label.toUpperCase()" />
</template>
