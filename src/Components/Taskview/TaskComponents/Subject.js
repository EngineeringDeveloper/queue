import React from "react"
import "./Subject.css"

const Subject = ({ subject , onClick}) => {
    return <div className='Subject' onClick={onClick}>{subject}</div>;
};
  
export default Subject