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

  return (
    <li key={props.details.priority} className='Task'>
      <Checkbox></Checkbox>
      <div>
        {"subject :"}
        <div>{props.details.subject}</div>
      </div>

      <div className='Project'>
        {"Project :"}
        <div>{props.details.projects}</div>
      </div>

      <div>
        <div>{"date :" + props.details.create_date}</div>
        <div>{"date_f :" + props.details.finish_date}</div>
        <div>{"fin :" + props.details.finished}</div>
        <div>{"due :" + props.details.due_date}</div>
        <div className='Context'>
          {"contexts :"}
          <div>{props.details.contexts}</div>
        </div>

        <div className='Hashtag'>
          {"hash :"}
          <div>{props.details.hashtags}</div>
        </div>

        <div>{tags}</div>
      </div>
    </li>
  );
}
