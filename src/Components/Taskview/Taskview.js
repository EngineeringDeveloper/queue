import React, { Component } from "react";
import Task from "./Task.js";
import Tabs from "../Tab/Tabs.js";
import NewToDoModal from "../Modal/NewToDoModal.js";
import "./Taskview.css";
import SearchIcon from "@mui/icons-material/Search";
import CircularProgress from "@mui/material/CircularProgress";
// https://mui.com/components/progress/
// import { invoke } from "@tauri-apps/api/tauri";

function genTaskListComponents(taskList_objects, setModal) {
  let outputArray = [];
  for (let taskList in taskList_objects) {
    taskList = taskList_objects[taskList];

    let taskVec = taskList.tasks;
    let source = taskList.source;
    let priority_sort = taskList.prioritised_tasks;

    let todo_list = [];
    if ((typeof taskVec !== "undefined") | (taskVec.length !== 0)) {
      for (let priority in priority_sort) {
        let taskList_slice = priority_sort[priority];
        priority = parseInt(priority);
        let priorityLetter =
          priority < 26 ? (priority + 10).toString(36).toUpperCase() : "None";

        let tasks = taskList_slice.map((content, index) => {
          return (
            <Task
              task={content}
              source={source}
              edit={setModal}
              key={index}
            ></Task>
          );
        });
        todo_list.push(
          <li className={`Priority ${priorityLetter}`} key={priority}>
            <div id='title'>{priorityLetter}</div>
            <ul>{tasks}</ul>
          </li>
        );
      }
    } else {
      todo_list = <div />;
    }

    outputArray.push(
      <div label={source}>
        <ul className="noList" >{todo_list}</ul>
      </div>
    );
    // outputArray.push(subArray);
    // }
  }

  return <Tabs>{outputArray}</Tabs>;
}

class Taskview extends Component {
  constructor(props) {
    super(props);
    this.state = {
      taskList: props.taskList,
      error: null,
      loading: true,
      showModal: false,
      modalTask: null,
      setShow: function (bool) {
        this.setState({ show: bool });
      },
    };
  }

  componentDidMount() {
    // If the component mounted then we evaluate if the promise resolved
    this.state.taskList
      .then((list) => {
        const setModalTask = (task) => {
          this.setState({ modalTask: task, showModal: true });
        };
        this.setState({ taskList: genTaskListComponents(list, setModalTask) });
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
        <NewToDoModal
          show={this.state.showModal}
          task={this.state.modalTask}
          onClose={() => {
            this.setState({ showModal: false , modalTask: null});
          }}
        />
        <header className='Header'>
          <SearchIcon />
          <button onClick={() => this.setState({ showModal: true })}>
            ShowModal
          </button>

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
