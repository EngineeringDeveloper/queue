import React, { Component } from "react";
import Task from "./Task.js";
import Tabs from "../Tabs.js";
import "./Taskview.css";
import SearchIcon from "@mui/icons-material/Search";
import CircularProgress from "@mui/material/CircularProgress";
// https://mui.com/components/progress/
import { invoke } from "@tauri-apps/api/tauri";

function genTaskListComponents(taskList_objects) {
  console.log(taskList_objects)
  let outputArray = [];
  for (let taskList in taskList_objects) {
    taskList = taskList_objects[taskList];
    
    let taskVec = taskList.tasks;
    let source = taskList.source;
    let priority_sort = taskList.prioritised_tasks;

    let todo_list = [];
    if (typeof taskVec !== 'undefined' | taskVec.length !== 0) {
      
      for (let priority in priority_sort) {
        let taskList_slice = priority_sort[priority]
        priority = parseInt(priority);
        let priorityLetter =
          priority < 26 ? (priority + 10).toString(36).toUpperCase() : "None";
        
        let tasks = taskList_slice.map((content, index) => {
          console.log(content.input_hash)
          return (
            <Task task={content} source={source}></Task>
            )
        })
        todo_list.push(
          <li className={`Priority ${priorityLetter}`} key={priority}>
            <div id='title'>{priorityLetter}</div>
              <ul>
                {tasks}
              </ul>
          </li>
        );
      }
    } else {
      console.log(typeof taskVec, taskVec.length)
      todo_list = (<div/>)
    }

    outputArray.push(
      <div label={source}>
        <ul>
        {todo_list}
        </ul>
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
        console.log("list")
        console.log(list)
        console.log("list")
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
        <div id='icon'>
          <CircularProgress color='inherit' />
        </div>
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
