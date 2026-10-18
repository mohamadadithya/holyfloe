<script setup lang="ts">
import { onMounted, ref, useTemplateRef } from 'vue';
import { FxButton } from '../../../fixtures/out/vue/fx-button/index.js';
import { log } from '../../shared/log.ts';

const disabled = ref(true);
const renders = ref(0);
const variant = ref<'danger' | undefined>('danger');
const button = useTemplateRef<InstanceType<typeof FxButton>>('button');
onMounted(() => {
  window.refs = { button: button.value?.element ?? null };
});
</script>

<template>
  <button id="toggle" @click="disabled = !disabled">toggle disabled</button>
  <button id="rerender" @click="renders++">rerender {{ renders }}</button>
  <button id="unset-variant" @click="variant = undefined">unset variant</button>
  <FxButton id="btn" ref="button" :variant="variant" :disabled="disabled">Go</FxButton>
  <FxButton id="btn2" @fx-press="(e) => log(`press:${renders}:${e.currentTarget.tagName}`)">Click</FxButton>
  <FxButton id="plain" class="extra" data-x="1" @click="log('native click')">Plain</FxButton>
</template>
