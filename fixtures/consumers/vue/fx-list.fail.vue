<!-- Each marked element must fail to type-check. -->
<script setup lang="ts">
import { FxList } from '../../out/vue/fx-list/index.js';
</script>

<template>
  <!-- @vue-expect-error items need id and label strings -->
  <FxList :items="[{ id: 1 }]" />
  <!-- @vue-expect-error dense is boolean -->
  <FxList :options="{ dense: 'yes' }" />
  <!-- @vue-expect-error renderLabel returns a string -->
  <FxList :render-label="() => 42" />
</template>
