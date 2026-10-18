<!-- Each marked element must fail to type-check. -->
<script setup lang="ts">
import { FxLazy } from '../../out/vue/fx-lazy/index.js';
</script>

<template>
  <!-- @vue-expect-error items are strings -->
  <FxLazy :items="[1]" />
</template>
