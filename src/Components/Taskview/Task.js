import React from "react";
import Checkbox from "@mui/material/Checkbox";

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
  console.log(typeof props.details.tags);
//   console.log(typeof props.details.priority);
  return (
    <li key={props.details.priority} className='Task'>
      <Checkbox></Checkbox>
      {/* <div>{"priority :"}</div> */}
      <div>{props.details.priority}</div>
      <div>{"subject :"}</div>
      <div>{props.details.subject}</div>
      <div>{"Project :"}</div>
      <div>{props.details.projects}</div>
      <div>
        <div>{"date :"}</div>
        <div>{props.details.create_date}</div>
        <div>{"date_f :"}</div>
        <div>{props.details.finish_date}</div>
        <div>{"fin :"}</div>
        <div>{props.details.finished}</div>
        <div>{"due :"}</div>
        <div>{props.details.due_date}</div>
        <div>{"contexts :"}</div>
        <div>{props.details.contexts}</div>
        <div>{"hash :"}</div>
        <div>{props.details.hashtags}</div>
        <div>{"tags :"}</div>
        {/* {props.details.tags.map((value, key) => {
                    return (<div> value </div>)
            })} */}
      </div>
    </li>
  );
}
