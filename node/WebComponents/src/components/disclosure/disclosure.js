import style from './style.scss';
import html from './template.html';

const template = document.createElement('template');

template.innerHTML = `<style>${style}</style>${html}`;

export class Disclosure extends HTMLElement {
  constructor() {
    super();

    this.attachShadow({mode: 'open'});
    this.shadowRoot.appendChild(template.content.cloneNode(true));

    this._button = this.shadowRoot.getElementById('button');
    this._content = this.shadowRoot.getElementById('content');
  }

  connectedCallback() {
    this._button.setAttribute('aria-controls', this._content.id);

    this._button.addEventListener('click', this._clickHandler.bind(this));

    this.hide();
  }

  get expanded() {
    const bool = this._button.getAttribute('aria-expanded');

    if (!bool) {
      return false;
    }

    return bool.toLowerCase() === 'true';
  }

  toggle() {
    this[this.expanded ? 'hide' : 'show']();
  }

  show() {
    this._content.classList.add('is-show');
    this._button.setAttribute('aria-expanded', 'true');
  }

  hide() {
    this._content.classList.remove('is-show');
    this._button.setAttribute('aria-expanded', 'false');
  }

  _clickHandler(event) {
    event.preventDefault();

    this.toggle();
  }
}
