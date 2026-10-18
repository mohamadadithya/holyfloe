declare global {
  interface Window {
    /** Everything the app's event handlers saw, in order. Tests read it. */
    log: string[];
    /** Elements the wrappers' refs point at, set after every render. */
    refs: Record<string, Element | null>;
  }
}

window.log = [];
export const log = (line: string) => window.log.push(line);
