use std::fs;
fn refactor(){
	let raw:String=fs::read_to_string("osracg_data.txt").unwrap();
	let mut rpline:Vec<String>=raw.clone().split("\n").map(|x|x.to_string()).collect::<Vec<String>>();
	let mut time_l:Vec<f64>=Vec::new();
	let mut time_n:Vec<f64>=Vec::new();
	let mut event_l:Vec<String>=Vec::new();
	for i in 0..rpline.len(){
		if rpline[i].is_empty(){
			continue
		}
		let (event,time):(String,String)=(rpline[i].clone().split("@").map(|x|x.to_string()).collect::<Vec<String>>()[0].clone(),rpline[i].clone().split("@").map(|x|x.to_string()).collect::<Vec<String>>()[1].clone());
		let a=time.clone().split(" ").map(|x|x.to_string()).collect::<Vec<String>>()[3].split(",").map(|x|x.to_string()).collect::<Vec<String>>().join("").parse::<f64>().unwrap();
		let b=time.clone().split(" ").map(|x|x.to_string()).collect::<Vec<String>>()[5].split(",").map(|x|x.to_string()).collect::<Vec<String>>().join("").parse::<f64>().unwrap();
		time_l.push(a);
		time_n.push(b);
		event_l.push(event);//later use
	}
	let mut reft_l:Vec<f64>=Vec::new();//refacted time list
	reft_l.push(0.0);
	for i in 1..time_l.len(){
		let mut tempbruh:f64=(((999999999-time_n[i-1]as i64)+time_n[i]as i64)as f64/1000000000.0);
		if tempbruh>=1.0{
			tempbruh=((tempbruh*1000000000.0)as i64-1000000000)as f64/1000000000.0;
		}
		println!("{}",tempbruh);
		reft_l.push((time_l[i]-time_l[i-1])+tempbruh);
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
		finale.push(format!("{}:{}",(reft_l[i]).to_string(),event_l[i].clone()));
	}//now write back into the file
	fs::write("osracg_data.txt",finale.join("\n"));
}
fn generator(types:String,name:String){
	let raw:String=fs::read_to_string("osracg_data.txt").unwrap();
	let raw_l:Vec<String>=raw.clone().split("\n").map(|x|x.to_string()).collect::<Vec<String>>();
	let mut d_l:Vec<f64>=Vec::new();
	let mut a_l:Vec<String>=Vec::new();
	let mut p_l:Vec<String>=Vec::new();
	for i in raw_l{
		if !i.is_empty(){
			d_l.push(i.clone().split(":").map(|x|x.to_string()).collect::<Vec<String>>()[0].clone().parse::<f64>().unwrap());
			a_l.push(i.clone().split(":").map(|x|x.to_string()).collect::<Vec<String>>()[1].clone());
			p_l.push(i.clone().split(":").map(|x|x.to_string()).collect::<Vec<String>>()[2].clone());
		}
	}
	if types=="python"{
		let mut finale:Vec<String>=Vec::new();
		finale.push(format!("import pyautogui as py;\nimport time;\ndef {}():\n",name.clone()));
		for i in 0..d_l.len(){
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
			if p_l[i].contains("Unknown"){continue}
			if a_l[i]=="KeyPress"{
				//here start mapping out
			}else if a_l[i]=="KeyRelease"{
				//here start mapping out
			}else if a_l[i]=="ButtonPress"{
				//here start mapping out
			}else if a_l[i]=="ButtonRelease"{
				//here start mapping out
			}else if a_l[i]=="MouseMove"{
				//here start mapping out
			}
		}
	}
}
fn main(){
	refactor();
	generator("python".to_string(),"testing".to_string());
}
