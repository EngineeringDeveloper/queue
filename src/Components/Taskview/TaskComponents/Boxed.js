import { React } from "react";
import "./Boxed.css"

const Boxed = ({ array, cssClass }) => {
    if (array.length > 0) {
      let list = array.map((value, key) => {
        return (
          <div className={cssClass + " Boxed"} key={key}>
            {value}
          </div>
        );
      });
      return <div style={{ display: "flex" }}>{list}</div>;
    }
    return null;
};
  
export default Boxed