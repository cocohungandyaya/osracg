use std::fs;
fn refactor(){
	let raw:String=fs::read_to_string("osracg_data.txt").unwrap();
	let mut rpline:Vec<String>=raw.clone().split("\n").map(|x|x.to_string()).collect::<Vec<String>>();
	let mut time_l:Vec<i64>=Vec::new();
	let mut event_l:Vec<String>=Vec::new();
	for i in 0..rpline.len(){
		if rpline[i].is_empty(){
			continue
		}
		let (event,time):(String,String)=(rpline[i].clone().split("@").map(|x|x.to_string()).collect::<Vec<String>>()[0].clone(),rpline[i].clone().split("@").map(|x|x.to_string()).collect::<Vec<String>>()[1].clone());
		time_l.push(time.clone().split(" ").map(|x|x.to_string()).collect::<Vec<String>>()[3].split(",").map(|x|x.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap());
		event_l.push(event);//later use
	}
	let mut reft_l:Vec<i64>=Vec::new();//refacted time list
	reft_l.push(0);
	for i in 1..time_l.len(){
		reft_l.push(time_l[i]-time_l[i-1]);
	}
	/*
	KeyPress(Key),
	KeyRelease(Key),
	ButtonPress(Button),
	ButtonRelease(Button),
	MouseMove {
		x: f64,
		y: f64,
		},
	Wheel {
		delta_x: i64,
		delta_y: i64,
	},
	*/
	//python is pyautogui,rust use enigo
	//i dont want to make it support wheel at first so maybe just key,mouse,mousemove sth like that first
	for i in 0..event_l.len(){//here i think only need change mouse move from struct to MouseMove(x,y)(3 and7)
		if event_l[i].contains("MouseMove"){
			let temp:Vec<String>=event_l[i].clone().split(" ").map(|x|x.to_string()).collect::<Vec<String>>();
			let x:f64=temp[3].clone().split(",").collect::<Vec<_>>().join("").to_string().parse::<f64>().unwrap();
			let y:f64=temp[5].clone().parse::<f64>().unwrap();
			event_l[i]=format!("MouseMove({},{})",x,y);
		}
	}
	//now combine everything and format like Duration:Action:param
	for i in 0..event_l.len(){
		event_l[i]=event_l[i].split("(").collect::<Vec<_>>().join(":").split(")").collect::<Vec<_>>().join("").to_string();
	}//done second_refactor
	//now merge together
	let mut finale:Vec<String>=Vec::new();
	for i in 0..event_l.len(){
		finale.push(format!("{}:{}",reft_l[i].to_string(),event_l[i].clone()));
	}//now write back into the file
	fs::write("osracg_data.txt",finale.join("\n"));
}
fn main(){
	refactor();
}
