import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';

var header = React.createElement("h1", {className: 'greeting'}, 'Hello, World!');


document.querySelector('html').onclick = function() {
  alert('Ouch! Stop poking me!');
};

ReactDOM.render(
  header,
  document.getElementById('root')
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
