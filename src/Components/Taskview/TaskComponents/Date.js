import React from "react";
import CalendarTodayTwoToneIcon from "@mui/icons-material/CalendarTodayTwoTone";
import "./Date.css";

function Date(props) {
  const NotNull = (element) => element !== null;
  
  let Created;
  if (NotNull(props.task.create_date)) {
    Created = <div>{"Created :" + props.task.create_date}</div>;
  } else {
    Created = null;
  }
  console.log("Created", props.task.create_date, NotNull(props.task.create_date), Created)
  let Finished;
  if (NotNull(props.task.finish_date)) {
    Finished = <div>{"Finished :" + props.task.finish_date}</div>;
  } else {
    Finished = null;
  }

  let Due;
  if (NotNull(props.task.due_date)) {
    Due = <div>{"Due :" + props.task.due_date}</div>;
  } else {
    Due = null;
  }

  if ([Created, Finished, Due].some(NotNull)) {
    return (
      <div className='Dates'>
        <CalendarTodayTwoToneIcon></CalendarTodayTwoToneIcon>
        {Created}
        {Finished}
        {Due}
      </div>
    );
    
  } else {
    return (null);
  }

}

export default Date;
