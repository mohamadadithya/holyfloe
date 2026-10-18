<!-- Each marked element must fail to type-check. -->
<script setup lang="ts">
import { FxSelect } from '../../out/vue/fx-select/index.js';
</script>

<template>
  <!-- @vue-expect-error options are strings -->
  <FxSelect :options="[1, 2]" />
  <!-- @vue-expect-error fx-change detail.value is a string -->
  <FxSelect @fx-change="(e) => e.detail.value.toFixed()" />
</template>
