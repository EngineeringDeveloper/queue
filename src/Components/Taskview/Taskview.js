import React, { useState } from 'react';
import './Taskview.css'
import SearchIcon from '@mui/icons-material/Search';
import { invoke } from '@tauri-apps/api/tauri'


function Taskview  (){
    const [input, setInput] = useState(''); // '' is the initial state value
    invoke("parse_todo", { path: "path" }).then(taskList) => console.log(taskList);
// https://stackoverflow.com/questions/36683770/how-to-get-the-value-of-an-input-field-using-reactjs
        return (
            <div className="Taskview">
                <header className="Header">
                    {/* <div></div> */}
                        {/* <div> */}
                        <SearchIcon />
                        <input className="TaskviewSearch" type="search" value={input} onInput={e => setInput(e.target.value)}
                        placeholder="(A) Todo text +project @context due:2020-12-12 rec:d"/>
                        {/* </div> */}
                </header>
                <div className="Content">
                    <p>"Content"</p>
                    {taskList.map((key, val) => {
                        return (
                            <li key={key, val}
                                className="row">
                                {" "}
                                <div id="title">
                                    {val}
                                </div>
                            </li>
                        )
                    })}
                </div>
            </div>
        );
}

export default Taskview;