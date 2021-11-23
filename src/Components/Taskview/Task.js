import React from 'react'
import Checkbox from '@mui/material/Checkbox';

export default function Task(props) {
    return (
        <li key={props.key}className="Task">
            <Checkbox></Checkbox>
            <div>{props.details.subject}</div>
        </li>
    )
}
