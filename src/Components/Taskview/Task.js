import { React, useState} from "react";
import Checkbox from "@mui/material/Checkbox";
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
    <div  className={'Task ' + checked}  onClick={() => props.edit(props.task)}>
      <Checkbox checked={checked} onChange={handleChange}></Checkbox>
      <div style={{ padding: 0, display: "flex", flexFlow: "column" }} >
        <Subject subject={props.task.subject} />
        <div style={{ padding: 0, display: "flex", flexFlow: "row" }}>
          <Boxed array={props.task.projects} cssClass='Project' />
          <Boxed array={props.task.contexts} cssClass='Context' />
          <Boxed array = {props.task.hashtags} cssClass="Hashtag"/>
          {/* <Boxed array = {props.task.tags} cssClass="Tag"/> */}
        </div>
        <div className='Dates'>
          <div>{"date :" + props.task.create_date}</div>
          <div>{"date_f :" + props.task.finish_date}</div>
          {/* <div>{"fin :" + props.task.finished}</div> */}
          <div>{"due :" + props.task.due_date}</div>
        </div>
      </div>
    </div>
  );
}

const Subject = ({ subject }) => {
  return <div className='Subject'>{subject}</div>;
};

const Boxed = ({ array, cssClass }) => {
  if (array.length > 0) {
    let list = array.map((value, key) => {
      return (
        <div className={cssClass + " Boxed"} key={key}>
          {value}
        </div>
      );
    });
    return <div style={{ display: "flex" }}>{list}</div>;
  }
  return null;
};
