import React, { useState } from 'react';
import './Taskview.css'
import SearchIcon from '@mui/icons-material/Search';
import { invoke } from '@tauri-apps/api/tauri'


function Taskview  (){
    const [input, setInput] = useState(''); // '' is the initial state value
    // invoke("parse_todo", { path: "path" }).then((taskList) => taskList.map((key, val) => console.log(key)));
    // const taskList = invoke("parse_todo", { path: "path" }).then((taskList) => { return taskList });
    // console.log(taskList)
    // https://stackoverflow.com/questions/36683770/how-to-get-the-value-of-an-input-field-using-reactjs
        return (
            <div className="Taskview">
                <header className="Header">
                        <SearchIcon />
                        <input className="TaskviewSearch" type="search" value={input} onInput={e => setInput(e.target.value)}
                        placeholder="(A) Todo text +project @context due:2020-12-12 rec:d"/>
                </header>
                <div className="Content">
                    <p>"Content"</p>
                    <div id = "icon" className="Loading">
                        <SearchIcon />
                    </div>
                    {/* <li key={0}
                        className="row">
                        {fat[0].test}
                    </li> */}
                    {/* {invoke("parse_todo", { path: "path" }).then((taskList) => {
                        return taskList.map((value, key) => {
                            // console.log(key, value.subject)
                            return (
                                <li key={key}
                                    className="row">
                                    {value.subject}
                                    <div id="content">
                                    {value.subject}
                                </div>
                                </li>
                            )
                        })
                    })} */}
                </div>
            </div>
        );
}

export default Taskview;