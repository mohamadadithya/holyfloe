<!-- Renders every fixture through the wrappers `just gen-fixtures` wrote to fixtures/out/vue.
     Element ids match e2e/svelte5/src/App.svelte; tests/wrappers.spec.ts runs on both. -->
<script setup lang="ts">
import { defineFxLazy } from '@holyfloe/fixtures/fx-lazy/fx-lazy.js';
import { onMounted, ref, useTemplateRef } from 'vue';
import { FxBadge } from '../../../fixtures/out/vue/fx-badge/index.js';
import { FxButton } from '../../../fixtures/out/vue/fx-button/index.js';
import { FxCard } from '../../../fixtures/out/vue/fx-card/index.js';
import { FxDeprecated } from '../../../fixtures/out/vue/fx-deprecated/index.js';
import { FxDerived } from '../../../fixtures/out/vue/fx-derived/index.js';
import { FxDialog } from '../../../fixtures/out/vue/fx-dialog/index.js';
import { FxEvents } from '../../../fixtures/out/vue/fx-events/index.js';
import { FxInput } from '../../../fixtures/out/vue/fx-input/index.js';
import { FxLazy } from '../../../fixtures/out/vue/fx-lazy/index.js';
import { FxList } from '../../../fixtures/out/vue/fx-list/index.js';
import { FxMixin } from '../../../fixtures/out/vue/fx-mixin/index.js';
import { FxSelect } from '../../../fixtures/out/vue/fx-select/index.js';
import { log } from '../../shared/log.ts';

const renders = ref(0);
const disabled = ref(true);
const variant = ref<'danger' | undefined>('danger');
const count = ref(3);
const text = ref('start');
const open = ref(false);
const choice = ref('b');
const button = useTemplateRef<InstanceType<typeof FxButton>>('button');
const card = useTemplateRef<InstanceType<typeof FxCard>>('card');
const dialog = useTemplateRef<InstanceType<typeof FxDialog>>('dialog');
const mixin = useTemplateRef<InstanceType<typeof FxMixin>>('mixin');
onMounted(() => {
  window.refs = { button: button.value?.element ?? null, card: card.value?.element ?? null };
});
</script>

<template>
  <button id="rerender" @click="renders++">rerender {{ renders }}</button>
  <button id="toggle" @click="disabled = !disabled">toggle disabled</button>
  <button id="unset-variant" @click="variant = undefined">unset variant</button>
  <button id="inc" @click="count++">increment count</button>
  <button id="define-lazy" @click="defineFxLazy">define fx-lazy</button>
  <button id="open-dialog" @click="open = true">open dialog</button>
  <button id="show-dialog" @click="dialog?.show()">show() on the instance</button>
  <button id="toggle-focus" @click="mixin?.toggleFocus()">toggleFocus() on the instance</button>

  <FxButton id="btn" ref="button" :variant="variant" :disabled="disabled">Go</FxButton>
  <FxButton id="btn2" @fx-press="(e) => log(`press:${renders}:${e.currentTarget.tagName}`)">Click</FxButton>
  <FxButton id="plain" class="extra" data-x="1" @click="log('native click')">Plain</FxButton>

  <FxBadge id="badge" :count="count" tone="success" label="3 unread" for="inbox">new</FxBadge>

  <FxList
    id="list"
    :items="[{ id: 'a', label: 'Alpha' }, { id: 'b', label: 'Beta' }, { id: 'c', label: 'Gamma' }]"
    :options="{ dense: true, maxItems: 2 }"
    :render-label="(item) => item.label.toUpperCase()"
  />

  <FxEvents
    id="events"
    @my-change="(e) => log(`my-change:${e.detail.value}`)"
    @my-change-camel="(e) => log(`myChange:${e.detail.value}`)"
    @my-change-pascal="(e) => log(`MyChange:${e.detail.checked}`)"
    @MYEVENT="(e) => log(`MYEVENT:${e.detail.count}`)"
  />

  <FxCard id="card" ref="card" elevated style="--fx-card-padding: 3px; color: red">
    <template #header><h2>Head</h2></template>
    <p>Body</p>
    <template #footer><span>Foot</span></template>
  </FxCard>
  <FxCard id="card-plain"><p>Only a body</p></FxCard>

  <FxDerived id="derived" label="L" size="lg" loading @fx-ready="(e) => log(`ready:${e.currentTarget.tagName}`)">kid</FxDerived>

  <FxDeprecated id="dep" tone="error" kind="x" />

  <FxInput id="input" v-model="text" name="q" />
  <output id="input-state">{{ text }}</output>

  <FxDialog id="dialog" ref="dialog" v-model:open="open">
    Dialog body
    <template #footer><button>OK</button></template>
  </FxDialog>
  <output id="dialog-state">{{ open }}</output>

  <FxSelect id="select" v-model="choice" :options="['a', 'b', 'c']" />
  <output id="select-state">{{ choice }}</output>

  <FxMixin id="mixin" ref="mixin" prefix="> " focused ring-color="red" @focus-change="(e) => log(`focus:${e.currentTarget.focused}`)">
    Mixed
  </FxMixin>

  <FxLazy id="lazy" :items="['x', 'y']" label="Lazy" active />
</template>
