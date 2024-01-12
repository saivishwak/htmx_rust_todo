import {
  LitElement,
  css,
  html,
} from "https://cdn.jsdelivr.net/gh/lit/dist@3.1.1/all/lit-all.min.js";

export class TodoApp extends LitElement {
  static properties = {};

  static styles = css``;

  constructor() {
    super();
  }

  render() {
    return html` <link rel="stylesheet" href="assets/css/output.css" />
      <div
        class="flex flex-col min-h-[500px] bg-slate-300 border border-gray-200 shadow-sm rounded-md p-4 md:p-5"
      >
        <slot />
      </div>`;
  }
}
customElements.define("todo-app", TodoApp);
