<!-- Must compile: inherited props and event. -->
<script setup lang="ts">
import { FxDerived } from '../../out/vue/fx-derived/index.js';
</script>

<template>
  <FxDerived label="L" size="lg" loading @fx-ready="(e) => e.currentTarget.size">kid</FxDerived>
</template>
