import React from 'react'
import { func, string } from 'prop-types';
import styled from "styled-components"
// import  {useDarkMode} from "./useDarkMode"

const Button = styled.button`
  background: ${({ theme }) => theme.background};
  border: 2px solid ${({ theme }) => theme.margin};
  color: ${({ theme }) => theme.text};
  border-radius: 30px;
  cursor: pointer;
  font-size:0.8rem;
  padding: 0.6rem;
  }
/`;

const ThemeToggler = ({ theme, toggleTheme }) => {
    // const [themeToggler, mountedComponent] = useDarkMode();
  
    return (
        <Button onClick={toggleTheme} >
          Switch Theme
        </Button>
    );
};
ThemeToggler.propTypes = {
    theme: string.isRequired,
    toggleTheme: func.isRequired,
}

export default ThemeToggler;
