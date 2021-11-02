import React, { useState } from 'react';

// Dark & Light Themes
// https://www.smashingmagazine.com/2020/04/dark-mode-react-apps-styled-components/#globalstyles-component
// import {ThemeProvider} from "styled-components";
// import { GlobalStyles } from "./Components/globalStyles";
// import { lightTheme, darkTheme } from "./Components/Themes"

// Theme
import ThemeToggler from './Components/ThemeToggler';
import  {useDarkMode} from "./Components/useDarkMode"
// https://medium.com/swlh/setting-up-light-and-dark-mode-in-a-react-application-just-with-styles-7790dea22aed
import './styles/theme.css';

// Custom Components
import Sidebar from './Components/Sidebar/Sidebar'


function App() {
  // Theme toggling
  const [theme, themeToggler, mountedComponent] = useDarkMode();
  // const themeMode = theme === 'light' ? lightTheme : darkTheme;

  if(!mountedComponent) return <div/>
  return(
    <div className={`App ${theme}`}>
      <div>
        <Sidebar />
      </div>
      <div>
        {/* Theme toggling Button  */}
        <ThemeToggler theme={theme} toggleTheme={themeToggler}/>
      </div>
    </div>
    )
}

export default App;