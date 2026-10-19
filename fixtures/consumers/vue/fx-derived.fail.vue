<!-- Each marked element must fail to type-check. -->
<script setup lang="ts">
import { FxDerived } from '../../out/vue/fx-derived/index.js';
</script>

<template>
  <!-- @vue-expect-error size (inherited) is a union of strings -->
  <FxDerived size="xxl" />
  <!-- @vue-expect-error loading (inherited) is a boolean -->
  <FxDerived :loading="1" />
</template>
