import React from "react";
import "./NewToDoModal.css";
import Modal from "./Modal.js";
import Boxed from "../Taskview/TaskComponents/Boxed.js"

export default function NewToDoModal(props) {

  const placeholder = "(A) Todo text +project @context due:2020-12-12 rec:d";
  var inputtext = null;
  var taskDetails = null;
  if (props.task) {
    inputtext = props.task.subject;
    taskDetails = <div style={{ padding: 0, display: "flex", flexFlow: "row"}}>
    <Boxed array={props.task.projects} cssClass='Project' />
    <Boxed array={props.task.contexts} cssClass='Context' />
    <Boxed array = {props.task.hashtags} cssClass="Hashtag"/>
  </div>
  }

  const children =
    <div className="content">
      <input className="textInput" type="text" placeholder={placeholder} value={inputtext}/>
      {taskDetails}
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
