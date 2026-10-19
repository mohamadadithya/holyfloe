<!-- Each marked element must fail to type-check. -->
<script setup lang="ts">
import { FxDeprecated } from '../../out/vue/fx-deprecated/index.js';
</script>

<template>
  <!-- @vue-expect-error tone is a union of strings -->
  <FxDeprecated tone="loud" />
</template>
