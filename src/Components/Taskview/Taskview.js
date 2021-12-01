import React, { Component } from "react";
import Task from "./Task.js";
import Tabs from "../Tabs.js";
import "./Taskview.css";
import SearchIcon from "@mui/icons-material/Search";
import CircularProgress from "@mui/material/CircularProgress";
// https://mui.com/components/progress/
import { invoke } from "@tauri-apps/api/tauri";

function genTaskListComponents(taskList_objects) {
  let outputArray = [];
  console.log(taskList_objects)
  for (let taskList in taskList_objects) {
    taskList = taskList_objects[taskList];
    // console.log(taskList);
    // let subArray = [];
    let source = taskList.source;
    let taskVec = taskList.tasks;
    console.log(source, taskVec)
    // for (let task in taskVec) {
    // let priority = task.priority;
    // const taskVec = taskList[priority];
    let setup;
    if (typeof variable !== 'undefined') {
      setup = (taskVec.map((content, index) => {
        let priority = parseInt(content.priority);
        let prioritryLetter =
          priority < 26 ? (priority + 10).toString(36).toUpperCase() : "None";
        return (
          <li className={`Priority ${prioritryLetter}`} key={priority}>
            <div id='title'>{prioritryLetter}</div>
            <ul>
              <Task details={content} index={index}></Task>
            </ul>
          </li>
        );
      })
      )
    } else {
      setup = (<div/>)
    }
    outputArray.push(
      <div label={source}>
        {setup}
      </div>
    );
    // outputArray.push(subArray);
    // }
  }

  // console.log(outputArray);
  return <Tabs>{outputArray}</Tabs>;
}

class Taskview extends Component {
  constructor(props) {
    super(props);
    this.state = {
      input: "",
      taskList: props.taskList,
      error: null,
      loading: true,
    };
  }

  componentDidMount() {
    // If the component mounted then we evaluate if the promise resolved
    this.state.taskList
      .then((list) => {
        let task_keys = [];
        this.setState({ taskList: genTaskListComponents(list) });
      })
      .finally(() => {
        this.setState({ loading: false });
      });
  }

  // https://stackoverflow.com/questions/36683770/how-to-get-the-value-of-an-input-field-using-reactjs
  render() {
    let LoadedTasks;
    if (this.state.loading) {
      LoadedTasks = (
        <box id='icon'>
          <CircularProgress color='inherit' />
        </box>
      );
    } else {
      LoadedTasks = this.state.taskList;
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
        <ul className='Content'>{LoadedTasks}</ul>
      </div>
    );
  }
}

export default Taskview;
