<!-- Each marked element must fail to type-check (raw tags). -->
<template>
  <!-- @vue-expect-error variant is a union of strings -->
  <fx-button variant="huge" />
  <!-- @vue-expect-error count is a number -->
  <fx-badge count="1" />
  <!-- @vue-expect-error my-change detail.value is a string -->
  <fx-events @my-change="(e) => e.detail.value.toFixed()" />
  <!-- @vue-expect-error options are strings -->
  <fx-select :options="[1]" />
  <!-- @vue-expect-error not a known element -->
  <fx-nope />
</template>
