import React, { useEffect } from "react";
import { CSSTransition } from "react-transition-group";
import ReactDOM from "react-dom";
import "./Modal.css";

export default function NewToDoModal(props) {
  useEffect(() => {
    const closeOnEscapeDown = (e) => {
      if ((e.charCode || e.keyCode) === 27) {
        props.onClose();
      }
    };
    document.body.addEventListener("keydown", closeOnEscapeDown);
    return function cleanup() {
      document.body.removeEventListener("keydown", closeOnEscapeDown);
    };
  });

  const placeholder = "(A) Todo text +project @context due:2020-12-12 rec:d";
  var inputtext = null;
  if (props.task) {
    inputtext = props.task.subject;
  }

  // also props can contain a task, in which case the fields should be filled
  const app = document.getElementById("app");
  if (app) {
    return ReactDOM.createPortal(
      <CSSTransition
        in={props.show}
        unmountOnExit
        timeout={{ enter: 0, exit: 300 }}
      >
        <div className={`modal`} onClick={props.onClose}>
          <div className='modal-content' onClick={(e) => e.stopPropagation()}>
            <div className='modal-header'>
              <h4 className='modal-title'>Modal Title</h4>
            </div>
            <div className='modal-body'>
              <input
                className='taskInput'
                type='text'
                placeholder={placeholder}
                value={inputtext}
              />
            </div>
            <div className='modal-footer'>
              <button className='button' onClick={props.onClose}>
                Close
              </button>
            </div>
          </div>
        </div>
      </CSSTransition>,
      app
    );
  } else {
    return null;
  }
}
