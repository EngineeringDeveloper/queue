import React, { useEffect } from "react";
import { CSSTransition } from "react-transition-group";
import ReactDOM from "react-dom"
import "./modal.css";

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
//     https://medium.com/tinyso/how-to-create-a-modal-component-in-react-from-basic-to-advanced-a3357a2a716a
  // if (!props.show) {
  //     return null
  // }

  return ReactDOM.createPortal(
    <CSSTransition
      in={props.show}
      unmountOnExit
      timeout={{ enter: 0, exit: 300 }}
    >
      <div
        className={`modal`}
        // className={`modal ${props.show ? "show" : ""}`}
        onClick={props.onClose}
      >
        <div className='modal-content' onClick={(e) => e.stopPropagation()}>
          <div className='modal-header'>
            <h4 className='modal-title'>Modal Title</h4>
          </div>
          <div>
            <h4 className='modal-body'> CONTENT </h4>
          </div>
          <div className='modal-footer'>
            <button className='button' onClick={props.onClose}>
              Close
            </button>
          </div>
        </div>
      </div>
    </CSSTransition>,
    document.getElementById('root')
  );
}
