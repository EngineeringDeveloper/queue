
# Queue
<div align="center">
![Queue](/images/Queue_icon.svg)
</div>
GUI Application for task and Job planning with priority consideration

<div align="center">

[![Build](https://github.com/EngineeringDeveloper/queue/actions/workflows/build.yml/badge.svg)](https://github.com/EngineeringDeveloper/queue/actions/workflows/build.yml)
[![Rust-Tests](https://github.com/EngineeringDeveloper/queue/actions/workflows/rust.yml/badge.svg)](https://github.com/EngineeringDeveloper/queue/actions/workflows/rust.yml)

</div>

Written in JavaScript with an React end & a back end in Rust via Tauri  
  
Uses as a base the "todo.txt" syntax  
![todo.txt](/images/todotxt_explainer.svg)

## References:  
https://github.com/todotxt/todo.txt  
https://dev.to/krofax/creating-desktop-apps-with-reactjs-using-tauri-af  
https://tauri.studio/en/docs/usage/development/development  
possible alternative for todo-txt  
https://github.com/VladimirMarkelov/todo_lib  

  
### Old
https://blog.logrocket.com/supercharge-your-electron-apps-with-rust/  
https://github.com/ransome1/sleek  
https://bit.dev/  

## Debugging Electron App

```bash
Ctrl + Shift + i
```


## Starting Development

Start the app in the `dev` environment:

```bash
yarn tauri dev
```

## Packaging for Production

To package apps for the local platform:

```bash
yarn tauri release
```
