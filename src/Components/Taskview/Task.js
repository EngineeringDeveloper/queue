import { React, useState} from "react";
import Checkbox from "@mui/material/Checkbox";
import "./Task.css";

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
  const [checked, setChecked] = useState(props.details.finished);
  const handleChange = () => {
    props.details.finished = !checked
    setChecked(!checked)
  }
  return (
    <li key={props.details.priority} className={'Task ' + checked}>
      <Checkbox checked={checked} onChange={handleChange}></Checkbox>
      <div style={{ padding: 0, display: "flex", flexFlow: "column" }}>
        <Subject subject={props.details.subject} />
        <div style={{ padding: 0, display: "flex", flexFlow: "row" }}>
          <Boxed array={props.details.projects} cssClass='Project' />
          <Boxed array={props.details.contexts} cssClass='Context' />
          <Boxed array = {props.details.hashtags} cssClass="Hashtag"/>
          <Boxed array = {props.details.tags} cssClass="Tag"/>
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

const Subject = ({ subject }) => {
  return <div className='Subject'>{subject}</div>;
};

const Boxed = ({ array, cssClass }) => {
  console.log(array.length > 0);
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
