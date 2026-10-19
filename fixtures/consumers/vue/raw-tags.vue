<!-- Must compile: raw tags typed by every global-components.d.ts. No wrapper is imported here:
     in a file that imports FxInput, `<fx-input>` resolves to the wrapper. -->
<template>
  <fx-button variant="primary" disabled @fx-press="(e) => e.currentTarget.disabled">raw</fx-button>
  <fx-badge :count="1" tone="warning" label="one" for="x" />
  <fx-card :elevated="false"><h2 slot="header">Raw</h2></fx-card>
  <fx-derived size="sm" @fx-ready="(e) => e.currentTarget.loading" />
  <fx-dialog open @open-change="(e) => e.detail.open === true" />
  <fx-events @my-change="(e) => e.detail.value.toUpperCase()" />
  <fx-input value="raw" name="r" @change="(e) => e.currentTarget.value.toUpperCase()" />
  <fx-lazy :items="['a']" label="L" active />
  <fx-list :items="[{ id: 'a', label: 'A' }]" />
  <fx-mixin focused ring-color="red" @focus-change="(e) => e.currentTarget.focused" />
  <fx-select value="a" :options="['a']" @fx-change="(e) => e.detail.value.toUpperCase()" />
  <fx-deprecated tone="info" />
</template>
