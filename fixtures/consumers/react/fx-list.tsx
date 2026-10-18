// Must compile: correct props, events, ref and children.
import type { ListItem } from '@holyfloe/fixtures/fx-list/types.js';
import { FxList } from '../../out/react/fx-list/index.js';

const items: ListItem[] = [{ id: 'a', label: 'Apple' }];

export function Consumer() {
  return <FxList items={items} options={{ dense: true, maxItems: 5 }} renderLabel={(item) => item.label.toUpperCase()} />;
}
