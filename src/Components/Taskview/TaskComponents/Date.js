import React from 'react'
import CalendarTodayIcon from '@mui/icons-material/CalendarToday';
import "./Date.css"

function Date(props) {
    return (
      <div className='Dates'>
          <CalendarTodayIcon></CalendarTodayIcon>
          <div>{"Created :" + props.task.create_date}</div>
          <div>{"Finished :" + props.task.finish_date}</div>
          {/* <div>{"fin :" + props.task.finished}</div> */}
          <div>{"Due :" + props.task.due_date}</div>
        </div>
    )
}

export default Date
