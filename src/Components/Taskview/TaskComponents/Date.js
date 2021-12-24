import React from 'react'

function Date(props) {
    return (
        <div className='Dates'>
          <div>{"date :" + props.task.create_date}</div>
          <div>{"date_f :" + props.task.finish_date}</div>
          {/* <div>{"fin :" + props.task.finished}</div> */}
          <div>{"due :" + props.task.due_date}</div>
        </div>
    )
}

export default Date
