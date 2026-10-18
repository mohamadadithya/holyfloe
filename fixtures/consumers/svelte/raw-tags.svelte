<!-- Must compile: raw tags typed by every svelte-elements.d.ts. The script block matters:
     svelte-check type-checks markup only in TypeScript components. -->
<script lang="ts"></script>

<fx-button variant="primary" disabled onfx-press={(e) => e.currentTarget.disabled}>raw</fx-button>
<fx-badge badge-count={1} tone="warning" label="one" for="x"></fx-badge>
<fx-card elevated={false}><h2 slot="header">Raw</h2></fx-card>
<fx-derived size="sm" onfx-ready={(e) => e.currentTarget.loading}></fx-derived>
<fx-dialog open onopen-change={(e) => e.detail.open === true}></fx-dialog>
<fx-events
  onmy-change={(e) => e.detail.value.toUpperCase()}
  onmyChange={(e) => e.detail.value.toFixed()}
  onMyChange={(e) => e.detail.checked === true}
  onMYEVENT={(e) => e.detail.count.toFixed()}
></fx-events>
<fx-input value="raw" name="r" onchange={(e) => e.currentTarget.value.toUpperCase()}></fx-input>
<fx-lazy items={['a']} label="L" active></fx-lazy>
<fx-list items={[{ id: 'a', label: 'A' }]}></fx-list>
<fx-mixin focused ring-color="red" onfocus-change={(e) => e.currentTarget.focused}></fx-mixin>
<fx-select value="a" options={['a']} onfx-change={(e) => e.detail.value.toUpperCase()}></fx-select>
<fx-deprecated tone="info"></fx-deprecated>
