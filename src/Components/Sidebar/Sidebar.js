import React from 'react';
import './Sidebar.css';
import {SidebarDataIcon, SidebarDataTop, SidebarDataBottom} from './SidebarData'

// https://www.youtube.com/watch?v=5R9jFHlG6ik&ab_channel=PedroTech
function Sidebar() {
    return(
        <div className="Sidebar">
            <ul className= "SidebarList">
                {SidebarDataIcon.map((val, key) => {
                    return (
                        <li key={key} 
                            className="row"
                            onClick={()=> {window.location.pathname = val.link}}>
                            <div id="icon">{val.icon}</div>
                        </li>
                    )
                })}
            </ul>
            <ul className= "SidebarList">
                {SidebarDataTop.map((val, key) => {
                    return (
                        <li key={key} 
                            className="row"
                            onClick={()=> {window.location.pathname = val.link}}>
                            {" "}
                            <div id="icon">{val.icon}</div>
                            {" "}
                            <div id="title">
                                {val.title}
                            </div>
                        </li>
                    )
                })}
            </ul>
            <div className="spacer"></div>
            <ul className= "SidebarList">
                {SidebarDataBottom.map((val, key) => {
                    return (
                        <li key={key} 
                            className="row"
                            onClick={()=> {window.location.pathname = val.link}}>
                            {" "}
                            <div>{val.icon}</div>
                            {" "}
                            <div>
                                {val.title}
                            </div>
                        </li>
                    )
                })}
            </ul>
        </div>
    )
}

export default Sidebar