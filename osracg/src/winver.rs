if !(major<=cmajor&&minor<=cminor&&patch<=cpatch){
					if major>cmajor{
						print!("A newer version is available:{} (current_version:{})\n",latest_version.clone(),current_version.clone());
						print!("Pulling the latest release...\n");
						let latest_link:String=format!("https://github.com/cocohungandyaya/osracg/releases/download/{}/osracg.exe",latest_version.clone());
						let latest_release=reqwest::get(latest_link.clone()).await.unwrap_or_else(|err|{eprint!("Error!\n{}",err);exit(1);}).bytes().await.unwrap_or_else(|err|{eprint!("Error!\n{}",err);exit(1);});
						print!("Done!\nNow renaming osracg into osracg_deprecated...\n");
						fs::rename(format!("{}/osracg.exe",current_executable.clone()),format!("{}/osracg_deprecated.exe",current_executable.clone())).unwrap();
						print!("Done!\nNow writing in the latest osracg...\n");
						fs::write(format!("{}/osracg.exe",current_executable.clone()),&latest_release).unwrap();
						print!("Done!");
					}else if major<=cmajor && minor>cminor{
						print!("A newer version is available:{} (current_version:{})\n",latest_version.clone(),current_version.clone());
						print!("Pulling the latest release...\n");
						let latest_link:String=format!("https://github.com/cocohungandyaya/osracg/releases/download/{}/osracg.exe",latest_version.clone());
						let latest_release=reqwest::get(latest_link.clone()).await.unwrap_or_else(|err|{eprint!("Error!\n{}",err);exit(1);}).bytes().await.unwrap_or_else(|err|{eprint!("Error!\n{}",err);exit(1);});
						print!("Done!\nNow renaming osracg into osracg_deprecated...\n");
						fs::rename(format!("{}/osracg.exe",current_executable.clone()),format!("{}/osracg_deprecated.exe",current_executable.clone())).unwrap();
						print!("Done!\nNow writing in the latest osracg...\n");
						fs::write(format!("{}/osracg.exe",current_executable.clone()),&latest_release).unwrap();
						print!("Done!");
					}else if major <=cmajor && minor<=cminor && patch>cpatch{
						print!("A newer version is available:{} (current_version:{})\n",latest_version.clone(),current_version.clone());
						print!("Pulling the latest release...\n");
						let latest_link:String=format!("https://github.com/cocohungandyaya/osracg/releases/download/{}/osracg.exe",latest_version.clone());
						let latest_release=reqwest::get(latest_link.clone()).await.unwrap_or_else(|err|{eprint!("Error!\n{}",err);exit(1);}).bytes().await.unwrap_or_else(|err|{eprint!("Error!\n{}",err);exit(1);});
						print!("Done!\nNow renaming osracg into osracg_deprecated...\n");
						fs::rename(format!("{}/osracg.exe",current_executable.clone()),format!("{}/osracg_deprecated.exe",current_executable.clone())).unwrap();
						print!("Done!\nNow writing in the latest osracg...\n");
						fs::write(format!("{}/osracg.exe",current_executable.clone()),&latest_release).unwrap();
						print!("Done!");
					}
				}else{print!("Current version({}) is the latest\n",current_version.clone());}
