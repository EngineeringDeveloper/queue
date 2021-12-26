import { React, useState} from "react";
import Checkbox from "@mui/material/Checkbox";
import Boxed from "./TaskComponents/Boxed.js"
import Date from "./TaskComponents/Date.js"
import Subject from "./TaskComponents/Subject.js"
import "./Task.css";

import { invoke } from '@tauri-apps/api/tauri'


export default function Task(props) {
  const [checked, setChecked] = useState(props.task.finished);
  const handleChange = () => {
    props.task.finished = !checked
    invoke("recieve_task", {newTask: props.task, source: props.source})
    setChecked(!checked)
  }
  // const editTask = props.edit(props.task)


  return (
    <div  className={'Task ' + checked} >
      <Checkbox checked={checked} onChange={handleChange}></Checkbox>
      <div style={{ padding: 0, display: "flex", flexFlow: "column" }} >
        <Subject subject={props.task.subject} onClick={() => props.edit(props.task)}/>
        <div style={{ padding: 0, display: "flex", flexFlow: "row"}}>
          <Boxed array={props.task.projects} cssClass='Project' />
          <Boxed array={props.task.contexts} cssClass='Context' />
          <Boxed array = {props.task.hashtags} cssClass="Hashtag"/>
          {/* <Boxed array = {props.task.tags} cssClass="Tag"/> */}
        </div>
        <Date task ={props.task}></Date>
      </div>
    </div>
  );
}




