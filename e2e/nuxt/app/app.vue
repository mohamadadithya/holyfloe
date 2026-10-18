<!-- Server-rendered, then hydrated. Rendering every fixture also proves each element module imports
     in Node. Ids match e2e/sveltekit/src/routes/+page.svelte; tests/ssr.spec.ts runs on both. -->
<script setup lang="ts">
import { ref } from 'vue';
import { FxBadge } from '../../../fixtures/out/vue/fx-badge/index.js';
import { FxButton } from '../../../fixtures/out/vue/fx-button/index.js';
import { FxCard } from '../../../fixtures/out/vue/fx-card/index.js';
import { FxDeprecated } from '../../../fixtures/out/vue/fx-deprecated/index.js';
import { FxDerived } from '../../../fixtures/out/vue/fx-derived/index.js';
import { FxDialog } from '../../../fixtures/out/vue/fx-dialog/index.js';
import { FxEvents } from '../../../fixtures/out/vue/fx-events/index.js';
import { FxInput } from '../../../fixtures/out/vue/fx-input/index.js';
import { FxList } from '../../../fixtures/out/vue/fx-list/index.js';
import { FxMixin } from '../../../fixtures/out/vue/fx-mixin/index.js';
import { FxSelect } from '../../../fixtures/out/vue/fx-select/index.js';

const presses = ref(0);
const text = ref('start');
const open = ref(true);
const choice = ref('b');
</script>

<template>
  <main>
    <FxButton id="ssr-btn" variant="danger" disabled>Danger</FxButton>
    <FxButton id="client-btn" @fx-press="presses++">Press</FxButton>
    <output id="presses">{{ presses }}</output>
    <FxBadge id="ssr-badge" :count="3" tone="success" label="3 unread" for="inbox">new</FxBadge>
    <FxList id="ssr-list" :items="[{ id: 'a', label: 'Alpha' }]" :options="{ dense: true }" />
    <FxCard id="ssr-card" elevated style="--fx-card-padding: 3px">
      <template #header><h2>Head</h2></template>
      <p>Body</p>
    </FxCard>
    <FxDerived id="ssr-derived" label="L" size="lg" />
    <FxDeprecated id="ssr-dep" tone="error" />
    <FxEvents id="ssr-events" />
    <FxInput id="ssr-input" v-model="text" name="q" />
    <output id="ssr-input-state">{{ text }}</output>
    <FxDialog id="ssr-dialog" v-model:open="open">Dialog</FxDialog>
    <output id="ssr-dialog-state">{{ open }}</output>
    <FxSelect id="ssr-select" v-model="choice" :options="['a', 'b']" />
    <FxMixin id="ssr-mixin" ring-color="red" focused>Mixed</FxMixin>
  </main>
</template>
