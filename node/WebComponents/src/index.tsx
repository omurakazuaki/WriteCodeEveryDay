import React from 'react';
import ReactDOM from 'react-dom';

const container = document.getElementById('contents');

ReactDOM.render((
  <c-disclosure>
    <div slot="title">Title</div>
    <div slot="content">
      Hello web components
    </div>
  </c-disclosure>
), container);
