import React from 'react'

export default function Task(props) {
    return (
        <div className= "Task">
            <div>{props.details.subject}</div>
        </div>
    )
}
