<!-- Each marked element must fail to type-check. -->
<script setup lang="ts">
import { FxEvents } from '../../out/vue/fx-events/index.js';
</script>

<template>
  <!-- @vue-expect-error my-change detail.value is a string -->
  <FxEvents @my-change="(e) => e.detail.value.toFixed()" />
  <!-- @vue-expect-error MyChangePascal detail has checked, not value -->
  <FxEvents @my-change-pascal="(e) => e.detail.value" />
</template>
