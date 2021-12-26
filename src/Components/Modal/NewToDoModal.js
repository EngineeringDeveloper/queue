import React from "react";
import "./NewToDoModal.css";
import Modal from "./Modal.js";

export default function NewToDoModal(props) {
  const title = "no title";

  const placeholder = "(A) Todo text +project @context due:2020-12-12 rec:d";
  var inputtext = null;
  if (props.task) {
    inputtext = props.task.subject;
  }
  const children =
    <div className="content">
      <input className="textInput" type="text" placeholder={placeholder} />
    </div>

  const submit = (content) => {
    // props.submit(content);
    props.onClose();
  };

  return (
    <Modal
      children={children}
      submit={submit}
      submitLabel={"Save"}
      onClose={props.onClose}
      show={props.show}
    ></Modal>
  );
}
