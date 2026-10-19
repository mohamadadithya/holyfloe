<!-- Must compile: v-model with the value from detail.value. -->
<script setup lang="ts">
import { ref } from 'vue';
import { FxSelect } from '../../out/vue/fx-select/index.js';

const choice = ref('a');
</script>

<template>
  <FxSelect v-model="choice" :options="['a', 'b']" @fx-change="(e) => e.detail.value.toUpperCase()" />
</template>
