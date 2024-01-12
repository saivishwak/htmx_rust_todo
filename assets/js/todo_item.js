import {
  LitElement,
  css,
  html,
} from "https://cdn.jsdelivr.net/gh/lit/dist@3.1.1/all/lit-all.min.js";

export class TodoItem extends LitElement {
  static get properties() {
    return {
      value: { type: String, reflect: true },
      checked: {
        type: Boolean,
        reflect: true,
        hasChanged: (newVal, oldVal) => {
          console.log("ASDASDAS", oldVal, newVal);
          return true;
        },
      },
    };
  }
  static styles = css``;

  constructor() {
    super();
    this.checked = false;
    this.value = "";
  }

  handleChecked(e) {
    this.checked = !!e.target.checked;
    let event = new CustomEvent("todoitem#checked", {
      detail: {
        checked: this.checked,
      },
      bubbles: true,
      cancelable: false,
    });
    this.dispatchEvent(event);
  }

  // Render the UI as a function of component state
  render() {
    return html`<link rel="stylesheet" href="assets/css/output.css" />
      <div class="grid space-y-3">
        <div class="relative flex items-start">
          <div class="flex items-center h-5 mt-1">
            <input
              type="checkbox"
              @change="${this.handleChecked}"
              class="border-gray-200 rounded text-blue-600 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-gray-800 dark:border-gray-700 dark:checked:bg-blue-500 dark:checked:border-blue-500 dark:focus:ring-offset-gray-800"
            ></input>
          </div>
          <label class="ms-3">
            <span
              id="hs-checkbox-delete-description"
              class="block text-xl text-slate-800"
              >${this.value}</span
            >
          </label>
        </div>
      </div>`;
  }
}
customElements.define("todo-item", TodoItem);
