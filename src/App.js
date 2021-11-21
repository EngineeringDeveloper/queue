import React from 'react';

// basic React
// https://www.youtube.com/watch?v=dGcsHMXbSOA&ab_channel=DevEd

// Dark & Light Themes
// https://www.smashingmagazine.com/2020/04/dark-mode-react-apps-styled-components/#globalstyles-component
// import {ThemeProvider} from "styled-components";
// import { GlobalStyles } from "./Components/globalStyles";
// import { lightTheme, darkTheme } from "./Components/Themes"

// Theme
import { useDarkMode } from "./Components/ThemeToggle/useDarkMode"
// import { ThemeProvider } from 'styled-components';
// https://medium.com/swlh/setting-up-light-and-dark-mode-in-a-react-application-just-with-styles-7790dea22aed
import './styles/theme.css';
import './App.css'

// Custom Components
import Sidebar from './Components/Sidebar/Sidebar'
import Taskview from './Components/Taskview/Taskview'
import NewToDoModal from './Components/NewToDoModal/NewToDoModal'
// import { GlobalStyles } from '@mui/styled-engine';

// Tauri Commands
// https://tauri.studio/en/docs/usage/guides/command
// With the Tauri API npm package:
// import { invoke } from '@tauri-apps/api/tauri'
// With the Tauri global script, enabled when `tauri.conf.json > build > withGlobalTauri` is set to true:


function App() {
  // Theme toggling
  const [theme, themeToggler, mountedComponent] = useDarkMode();
  // const invoke =  window.__TAURI__.invoke
  // const themeMode = theme === 'light' ? lightTheme : darkTheme;

  if (!mountedComponent) return <div />
  
  return (
      <div className={`App ${theme}`}>
        <div className="Row">
          <Sidebar theme={theme} toggleTheme={themeToggler} />
          <button onClick={console.log("help")} >help</button>
          <NewToDoModal />
          <Taskview />
        </div>
        </div>
    )
}

export default App;