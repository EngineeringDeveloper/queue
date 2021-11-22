import React, { Component, Suspense, useState } from "react";
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
    // console.log(showLoad(this.state))
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
    this.state.taskList
        .then((list) => {
        
        // console.log(list)
        this.setState({ taskList: list.map((value, key) => {
            // console.log(key, value.subject)
            return (
                <li key={key} className='row'>
                    <div id='content'>{value.subject}</div>
                </li>
            );
        })});
    })
    .finally(() => {
        this.setState({ loading: false } );
        // console.log(this.props.taskList)
    });
  }

  // https://stackoverflow.com/questions/36683770/how-to-get-the-value-of-an-input-field-using-reactjs
    render() {
        let loading;
        if (this.state.loading) {
            console.log(this.state.loading);
            loading = <box id="icon" ><CircularProgress color="inherit"/></box>;
        } else {
            console.log(this.state)
            // let list = this.props.taskList
            loading = this.state.taskList
            // console.log(list);
            // loading = list.map((value, key) => {
            //     console.log(key, value.subject)
            //     return (
            //         <li key={key} className='row'>
            //             <div id='content'>{value.subject}</div>
            //         </li>
            //     );
            // });
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
                    {loading}
                    {/* <showLoad loading={this.state.loading}></showLoad> */}
                    {/* <div id="icon" className='Loading'>
                        <SearchIcon />
                    </div> */}
                
                    {/* <div id="icon" className='Loading'>
                    <SearchIcon />
                    </div> */}
              {/* <Suspense id='icon' fallback={<SearchIcon className='Loading' />}> */}
                
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
              {/* </Suspense> */}
            </ul>
          </div>
        );
  }
}



export default Taskview;
