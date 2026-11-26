#![windows_subsystem = "windows"]
use rdev::{listen, Event};
use rdev::EventType::KeyPress;
use std::fs;
use std::process::exit;
use rdev::Key::*;
use std::time::SystemTime;
use std::thread;
use std::time::Duration;
use std::io::{Write,BufWriter};

// for what will do here is that everytime detect the uhh action and then open the file and edit and close the file that is it and pass to the generator which gerneate the file to the target location
fn main(){
	fs::write("osracg_data.txt","");
	if let Err(error) = listen(callback) {
	    println!("Error: {:?}", error)
	}
}

fn callback(event: Event) {
	//use fs::read_to_string and fs::write to append data into the file
	match event.event_type{//make a exit first which use F12 as exit then
	KeyPress(k)=>{
		if k==F12{
			exit(0)
		}else{
		let file=fs::OpenOptions::new().create(true).append(true).open("osracg_data.txt").unwrap();
		let mut writer=BufWriter::new(file);
		write!(writer,"{}",format!("{:?}@{:?}\n",event.event_type,SystemTime::now()).as_str()).unwrap();
		}
	},
	_=>{
		let file=fs::OpenOptions::new().create(true).append(true).open("osracg_data.txt").unwrap();
		let mut writer=BufWriter::new(file);
		write!(writer,"{}",format!("{:?}@{:?}\n",event.event_type,SystemTime::now()).as_str()).unwrap();
	}
	}
}
//i think done,what last is the generation need do the heavy lifting i guess,i want touch this ass as less as i can cuz kinda annoying
