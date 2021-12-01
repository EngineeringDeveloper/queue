import { React, useState} from "react";
import Checkbox from "@mui/material/Checkbox";
import "./Task.css";

import { invoke } from '@tauri-apps/api/tauri'


export default function Task(props) {
  console.log(props)
  const [checked, setChecked] = useState(props.details.finished);
  const handleChange = () => {
    props.details.finished = !checked
    invoke("recieve_task", {task: props.details, index: props.index})
    setChecked(!checked)
  }
  return (
    <div  className={'Task ' + checked}>
      <Checkbox checked={checked} onChange={handleChange}></Checkbox>
      <div style={{ padding: 0, display: "flex", flexFlow: "column" }}>
        <Subject subject={props.details.subject} />
        <div style={{ padding: 0, display: "flex", flexFlow: "row" }}>
          <Boxed array={props.details.projects} cssClass='Project' />
          <Boxed array={props.details.contexts} cssClass='Context' />
          <Boxed array = {props.details.hashtags} cssClass="Hashtag"/>
          {/* <Boxed array = {props.details.tags} cssClass="Tag"/> */}
        </div>
        <div className='Dates'>
          <div>{"date :" + props.details.create_date}</div>
          <div>{"date_f :" + props.details.finish_date}</div>
          {/* <div>{"fin :" + props.details.finished}</div> */}
          <div>{"due :" + props.details.due_date}</div>
        </div>
      </div>
    </div>
  );
}

const Subject = ({ subject }) => {
  return <div className='Subject'>{subject}</div>;
};

const Boxed = ({ array, cssClass }) => {
  console.log(cssClass, array)
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
