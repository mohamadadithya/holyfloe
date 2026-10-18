<!-- Must compile: complex, primitive and boolean props. -->
<script setup lang="ts">
import { FxLazy } from '../../out/vue/fx-lazy/index.js';
</script>

<template>
  <FxLazy :items="['a', 'b']" label="Fruit" active />
</template>
