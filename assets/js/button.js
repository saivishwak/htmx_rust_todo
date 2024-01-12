import {
  LitElement,
  css,
  html,
} from "https://cdn.jsdelivr.net/gh/lit/dist@3.1.1/all/lit-all.min.js";

export class SimpleButton extends LitElement {
  static get properties() {
    return {
      label: { type: String, reflect: true },
      count: { type: Number },
    };
  }

  constructor() {
    super();
    // Declare reactive properties
    this.label = "";
    this.count = 0;
  }

  click() {
    this.count++;
  }

  // Render the UI as a function of component state
  render() {
    return html`<button @click="${this.click}">${this.label}</button>
      <p>${this.count}</p>`;
  }
}
customElements.define("simple-button", SimpleButton);
