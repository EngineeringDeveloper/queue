import React, { useState } from 'react';

// Dark & Light Themes
// https://www.smashingmagazine.com/2020/04/dark-mode-react-apps-styled-components/#globalstyles-component
import {ThemeProvider} from "styled-components";
import { GlobalStyles } from "./Components/globalStyles";
import { lightTheme, darkTheme } from "./Components/Themes"

// Custom Components
import Sidebar from './Components/Sidebar'



function App() {
  const [theme, setTheme] = useState('light');
  const themeToggler = () => {
    theme === 'light' ? setTheme('dark') : setTheme('light')
}

  return(
    <ThemeProvider theme={theme === 'light' ? lightTheme : darkTheme}>
      <>
      <GlobalStyles/>
    <div>
      {/* <Sidebar /> */}
      <button onClick={themeToggler}>Switch Theme</button>
    </div>
    </>
    </ThemeProvider>
  )
}

export default App;