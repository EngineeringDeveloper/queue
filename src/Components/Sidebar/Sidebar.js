import React from 'react';
import './Sidebar.css';
import ThemeToggler from '../ThemeToggler';
import {SidebarDataIcon, SidebarDataTop, SidebarDataBottom} from './SidebarData'

// https://www.youtube.com/watch?v=5R9jFHlG6ik&ab_channel=PedroTech
function Sidebar({ theme, toggleTheme }) {
    return(
        <div className="Sidebar" >
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
            <ThemeToggler theme={theme} toggleTheme={toggleTheme}/>
            {/* <div className="spacer"></div> */}
            <ul className= "SidebarListBottom">
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