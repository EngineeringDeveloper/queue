import React, { Component, Suspense, useState } from "react";
import "./Taskview.css";
import SearchIcon from "@mui/icons-material/Search";
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
  // const [input, setInput] = useState('input'); // '' is the initial state value
  // const [taskList, settaskList] = useState('taskList'); // '' is the initial state value
  // settaskList ( {
  //     taskList: null,
  //     error: null,
  //     loading: true,
  // });
  // invoke("parse_todo", { path: "path" }).then((taskList) => taskList.map((key, val) => console.log(key)));
  // const taskList = invoke("parse_todo", { path: "path" });

  componentDidMount() {
    this.taskList
      .then((taskList) => {
        this.setState({ taskList: taskList });
      })
      .catch()
      .finally(() => {
        this.setState({ loading: false });
      });
  }

  // console.log(taskList)
  // https://stackoverflow.com/questions/36683770/how-to-get-the-value-of-an-input-field-using-reactjs
    render() {
      if 
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
          <Suspense id='icon' fallback={<SearchIcon className='Loading' />}>
            {this.taskList.map((value, key) => {
              // console.log(key, value.subject)
              return (
                <li key={key} className='row'>
                  <div id='content'>{value.subject}</div>
                </li>
              );
            })}
            {/* {invoke("parse_todo", { path: "path" }).then((taskList) => {
                        // console.log(taskList[0].subject)
                        taskList.map((value, key) => {
                            // console.log(key, value.subject)
                            return (
                                <li key={key}
                                    className="row">
                                    <div id="content">
                                        {value.subject}
                                    </div>
                                </li>
                            )
                        })
                    })} */}
          </Suspense>
        </ul>
      </div>
    );
  }
}

export default Taskview;
