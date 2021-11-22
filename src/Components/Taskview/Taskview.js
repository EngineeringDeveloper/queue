import React, { Component} from "react";
import "./Taskview.css";
import SearchIcon from "@mui/icons-material/Search";
import CircularProgress from '@mui/material/CircularProgress';
// https://mui.com/components/progress/
import { invoke } from "@tauri-apps/api/tauri";

class Taskview extends Component {
  constructor(props) {
    super(props);
    this.state = {
      input: "",
      taskList: invoke("parse_todo", { path: "path" }),
      error: null,
      loading: true,
    };
    }

    componentDidMount() {
      // If the component mounted then we evaluate if the promise resolved
    this.state.taskList
        .then((list) => {
        this.setState({ taskList: list.map((value, key) => {
            return (
                <li key={key} className='row'>
                    <div id='content'>{value.subject}</div>
                </li>
            );
        })});
    })
    .finally(() => {
        this.setState({ loading: false } );
    });
  }

  // https://stackoverflow.com/questions/36683770/how-to-get-the-value-of-an-input-field-using-reactjs
    render() {
        let LoadedTasks;
        if (this.state.loading) {
            console.log(this.state.loading);
            LoadedTasks = <box id="icon" ><CircularProgress color="inherit"/></box>;
        } else {
            console.log(this.state)
            LoadedTasks = this.state.taskList
        }
    
        return (
          <div className='Taskview'>
            <header className='Header'>
              <SearchIcon />
              <input
                className='TaskviewSearch'
                type='search'
                value={this.input}
                onInput={(e) => (this.input = e.target.value)}
                placeholder='(A) Todo text +project @context due:2020-12-12 rec:d'
              />
            </header>
            <ul className='Content'>
                    <p>"Content"</p>
                    {LoadedTasks}
            </ul>
          </div>
        );
  }
}



export default Taskview;
