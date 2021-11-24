import React from "react";
import Checkbox from "@mui/material/Checkbox";
import "./Task.css";

function objHasContent(obj) {
  for (var i in obj) return true;
  return false;
}
// pub struct Simple {
//     pub subject: String,
//     #[cfg_attr(feature = "serde-support", serde(default = "Priority::lowest"))]
//     pub priority: u8,
//     pub create_date: Option<crate::Date>,
//     pub finish_date: Option<crate::Date>,
//     #[cfg_attr(feature = "serde-support", serde(default))]
//     pub finished: bool,
//     pub threshold_date: Option<crate::Date>,
//     pub due_date: Option<crate::Date>,
//     #[cfg_attr(feature = "serde-support", serde(default))]
//     pub contexts: Vec<String>,
//     #[cfg_attr(feature = "serde-support", serde(default))]
//     pub projects: Vec<String>,
//     #[cfg_attr(feature = "serde-support", serde(default))]
//     pub hashtags: Vec<String>,
//     #[cfg_attr(feature = "serde-support", serde(default))]
//     pub tags: BTreeMap<String, String>,
// }

export default function Task(props) {
  const tags = objHasContent(props.details.tags)
    ? props.details.tags.toString()
    : "";
  // console.log(props.details.projects)
  return (
    <li key={props.details.priority} className='Task'>
      <Checkbox></Checkbox>
      <div style={{ padding: 0, display: "flex", flexFlow: "column" }}>
        <div>
          <Subject subject={props.details.subject} />
          <Boxed array= {props.details.projects} cssClass="Project"/>
          {/* <div className='Project Boxed'>{props.details.projects}</div> */}
          <div className='Context Boxed'>{props.details.contexts}</div>
          <div className='Hashtag Boxed'>{props.details.hashtags}</div>
          <div>{tags}</div>
        </div>
        <div className='Dates'>
          <div>{"date :" + props.details.create_date}</div>
          <div>{"date_f :" + props.details.finish_date}</div>
          {/* <div>{"fin :" + props.details.finished}</div> */}
          <div>{"due :" + props.details.due_date}</div>
        </div>
      </div>
    </li>
  );
}

const Subject = ({subject}) => {
  return <div className='Subject'>{subject}</div>
}

const Boxed = ({ array, cssClass }) => {
  console.log(array.length > 0)
  if (array.length > 0) {
    let list = array.map((value, key) => {
    return (<li className={cssClass + " Boxed"} key={key}>{value}</li>)
    })
    return (<ul style={{display: "flex", flexFlow: "column"}}>{list}</ul>)
  }
  return null
}
