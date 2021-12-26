import React, { useEffect } from "react";
import ReactDOM from "react-dom";
import { CSSTransition } from "react-transition-group";
import "./Modal.css";

const Modal = (props) => {
  const closeOnEscapeKeyDown = (e) => {
    if ((e.charCode || e.keyCode) === 27) {
      props.onClose();
    }
  };

  useEffect(() => {
    document.body.addEventListener("keydown", closeOnEscapeKeyDown);
    return function cleanup() {
      document.body.removeEventListener("keydown", closeOnEscapeKeyDown);
    };
  });
    

  const app = document.getElementById("app");
  if (app) {
    return ReactDOM.createPortal(
      <CSSTransition
        in={props.show}
        unmountOnExit
        timeout={{ enter: 0, exit: 300 }}
      >
        <div className='modal' onClick={props.onClose}>
          <div className='modal-content' onClick={(e) => e.stopPropagation()}>
            <div className='modal-body'>{props.children}</div>
            <div className='modal-footer'>
              <button onClick={props.onClose} className='button'>
                Close
              </button>
              <button className="button" onClick={() => props.submit(props.content)}>{props.submitLabel}</button>
            </div>
          </div>
        </div>
      </CSSTransition>,
      app
    );
  } else {
    return null;
  }
};

export default Modal;
