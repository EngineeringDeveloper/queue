import React, { Component } from "react";
import Task from "./Task.js";
import "./Taskview.css";
import SearchIcon from "@mui/icons-material/Search";
import CircularProgress from "@mui/material/CircularProgress";
// https://mui.com/components/progress/
import { invoke } from "@tauri-apps/api/tauri";

// js function for grouping by an internal json value
function groupBy(list, keyGetter) {
  const map = new Map();
  list.forEach((item) => {
    const key = keyGetter(item);
    const collection = map.get(key);
    if (!collection) {
      map.set(key, [item]);
    } else {
      collection.push(item);
    }
  });
  const sortedMap = new Map([...map.entries()].sort());
  return sortedMap;
}

class Taskview extends Component {
  constructor(props) {
    super(props);
    this.state = {
      input: "",
      taskList: invoke("parse_todo", {
        path: "C:\\Users\\ryanc\\Documents\\Repositories\\queue\\testFiles\\todo.txt",
      }),
      error: null,
      loading: true,
    };
  }

  componentDidMount() {
    // If the component mounted then we evaluate if the promise resolved
    this.state.taskList
      .then((list) => {
        // group the list by priority levels
        // let sortedList = list.sort((a, b) => a.priority - b.priority);
        // this.setState({
        //   taskList: sortedList.map((value, key) => {
        //     return <Task details={value} key={key}></Task>;
        //   }),
        // });
        let groupedMap = groupBy(list, (obj) => obj.priority);
        console.log(groupedMap)
        // groupedMap = groupedMap.sort((a, b) => a.key - b.key)
        let outputArray = [];
        for (let [priorityOrder, subList] of groupedMap) {
          // console.log(priorityOrder)
          outputArray.push(
            <li className="Priority" key={priorityOrder}>
              <div>{priorityOrder < 26 ? (priorityOrder + 10).toString(36).toUpperCase() : "No Priority"}</div>
              <ul>
                {subList.map((content, key) => {
                  return <Task details={content} key={key}></Task>;
                })}
              </ul>
            </li>
          );
        }
        console.log(outputArray)
        this.setState({ taskList: outputArray });
        // let sortedList = list.sort((a, b) => a.priority - b.priority);
        // this.setState({
        //   taskList: groupedList.map((subList, priorityOrder) => {
        //     return (
        //       <ul key={priorityOrder}>
        //         {subList.map((content, key) => {
        //           return <Task details={content} key={key}></Task>;
        //         })}
        //       </ul> // could be Prority wrapper
        //     );
        //   }),
        // });
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
