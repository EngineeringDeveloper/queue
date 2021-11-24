// This File will manage updates similar to VScode
// Automatically downloading the update if there is a newer release
// this is then cached, and the exe is replaced on exit
// The latest release is tested with the current version and the latest release on github

// possibly use https://docs.rs/self_update/0.27.0/self_update/index.html

// use reqwest::{Client};
// use reqwest::header::USER_AGENT;
// use serde_json;
// // import for temp file locations
// use tempfile::Builder;
// use std::io::copy;
// use std::fs::File;
// use bytes::Buf;
#![allow(dead_code)]

const USER: &str = "EngineeringDeveloper";
const REPO: &str = "queue";


pub fn check_update() -> bool{
    // Checks if there is a newer released than the one installed
    true
}

pub fn download_update() {
    // called when checkUpdate == True
    // makes async request to github for the latest install file
    // How can we have load this latest version
}

pub fn install_on_close() {
    // Function that is called on close of queue
    // calls some installer file to update the files after the program is not running
    // like vscode!
}

