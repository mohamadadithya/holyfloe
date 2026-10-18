<!-- Plain tags, no wrapper (OQ-003): what Vue's own v-model and prop handling do on a custom element. -->
<script setup lang="ts">
import '@holyfloe/fixtures/fx-input/fx-input.js';
import '@holyfloe/fixtures/fx-lazy/fx-lazy.js';
import { defineFxLazy } from '@holyfloe/fixtures/fx-lazy/fx-lazy.js';
import { ref } from 'vue';

const text = ref('start');
</script>

<template>
  <button id="define-lazy" @click="defineFxLazy">define fx-lazy</button>
  <fx-input id="raw-input" v-model="text"></fx-input>
  <output id="raw-input-state">{{ text }}</output>
  <fx-lazy id="raw-lazy" :items="['p', 'q']" label="Raw"></fx-lazy>
</template>
