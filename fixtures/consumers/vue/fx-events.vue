<!-- Must compile: every casing through its base name, with typed detail. -->
<script setup lang="ts">
import { FxEvents } from '../../out/vue/fx-events/index.js';
</script>

<template>
  <FxEvents
    @my-change="(e) => e.detail.value.toUpperCase()"
    @my-change-camel="(e) => e.detail.value.toFixed()"
    @my-change-pascal="(e) => e.detail.checked === true"
    @MYEVENT="(e) => e.detail.count.toFixed()"
  />
</template>
