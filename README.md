# osracg
A tools(CLI/GUI) to auto generate python/rust code file after recorded os keyboard and mouse actions

The file got generated is a standalone file which you can choose to rather import or run it

Full name:Os recorder action code generator

## Progress Log:
21 Nov 2026:Done the core part(press F12 to end recording),i also done the refactor part which format everyting into Duration:Action:param,also i make adjustment so then it read 0.1 seconds instead of 1 full seconds

22 Nov 2026:Done mapping out keys that will support and initialize the function generator,because i want ~play minecraft~ and today go hiking not much time to code so ~that is it today~ fahhh i spend 2 hours on fixing the refactor to support nanoseconds(which will be used for execution time matching the recording time duration between each action),no time for minecraft today :(

24 Nov 2026:Completer the first ever release of osracg(v0.1.0) which currently only support naming your own module and generating python code file,also what i do is i make osracg only generate useful movements,making it run more smooth and matching the real timing when you click
## Project Direction:
core->refactor->generator(involved env)->osracg(involved env)(CLI)->gui(tkinter)
*the refactor and generator both embed into the osracg(CLI)

## Version Notes
v0.1.0: First edition,support for generation of python code and naming your own module

# Quick Start
Requirements:python installed,pip installed pyautogui

1. Download the release that match your system and unzip it
2. Run the osracg with your terminal/CMD opened in the folder

  Linux:
  ```terminal
  ./osracg Your_module_name
  ```
  windows:
  ```terminal
  osracg.exe Your_module_name
  ```
3. it will auto perform Alt-Tab so remember to put the page you want to record in the background first
4. After you done recording press F12 to end the recording and it will auto generate a file call osracg.py
5. To use the code file,you can rather run it directly or drop it to the same folder next to your python code file,to use it:
```py
import osracg
osracg.your_module_name()
```
For details on what keys osraccg support,check out [Support_keys.txt](https://github.com/cocohungandyaya/osracg/blob/main/Support_Keys.txt)

Note:F12(not support for all version because it is used for stop recording) and mouse wheel is not support in current version(v0.1.0)
