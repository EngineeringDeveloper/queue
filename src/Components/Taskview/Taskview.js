import React, { Component } from "react";
import Task from "./Task.js";
import "./Taskview.css";
import SearchIcon from "@mui/icons-material/Search";
import CircularProgress from "@mui/material/CircularProgress";
// https://mui.com/components/progress/
import { invoke } from "@tauri-apps/api/tauri";

// // js function for grouping by an internal json value
// function groupBy(list, keyGetter) {
//   const map = new Map();
//   list.forEach((item) => {
//     const key = keyGetter(item);
//     const collection = map.get(key);
//     if (!collection) {
//       map.set(key, [item]);
//     } else {
//       collection.push(item);
//     }
//   });
//   const sortedMap = new Map([...map.entries()].sort());
//   return sortedMap;
// }

function genTaskListComponents(taskList) {
  // Creates a Priority grouped and sorted list of the task list
  // let groupedMap = groupBy(taskList, (obj) => obj.priority);
  let outputArray = [];
  for (let priority in taskList) {
    if (taskList.hasOwnProperty(priority)) {
      const taskVec = taskList[priority];
      priority = parseInt(priority);
      let prioritryLetter =
        priority < 26 ? (priority + 10).toString(36).toUpperCase() : "None";
      outputArray.push(
        <li className={`Priority ${prioritryLetter}`} key={priority}>
          <div id='title'>{prioritryLetter}</div>
          <ul>
            {taskVec.map((content, index) => {
              return <Task details={content} index={index}></Task>;
            })}
          </ul>
        </li>
      );
    }
  }

  console.log(outputArray);
  return outputArray;
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
        console.log("test", list)
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
