# osracg
A tool (CLI/GUI) to auto generate python/rust code file after recorded os keyboard and mouse actions

The file got generated is a standalone file which you can choose to either import or run it

Full name:Os recorder action code generator
# Quick Start
Requirements:python installed,pip installed pyautogui

1. Download the release that match your system and unzip it
2. Run the osracg with your terminal/CMD opened in the folder

  Linux:
  ```terminal
  ./osracg your_file_name
  ```
  windows(CMD):
  ```terminal
  osracg.exe your_file_name
  ```
  windows(Powershell):
  ```powershell
  .\osracg your_file_name
  ```
3. it will auto perform Alt-Tab(This action won't be included in the generated file)so remember to put the page you want to record in the background first
4. After you done recording press F12 to end the recording and it will auto generate a file call your_file_name.py
5. To use the code file,you can rather run it directly or drop it to the same folder next to your python code file,to use it:
```py
import your_file_name
your_file_name.osracg()
```
For details on what keys osracg support,check out [Support_keys](https://github.com/cocohungandyaya/osracg/blob/main/Support_Keys.txt)

### Note that mouse Wheel is not supported

## Additional command:
Linux:
```terminal
./osracg -v
./osracg -h
```
Windows(CMD):
```terminal
osracg.exe -v
osracg.exe -h
```
Windows(PowerShell):
```powershell
.\osracg -v
.\osracg -h
```
For details on what is new in each version check out [Version Note](https://github.com/cocohungandyaya/osracg/blob/main/Version%20Note.txt)
### Not suggested to rely on holding down a key(e.g.Delete) because the code replay might have small inaccuracy
Here is a demo video for v0.1.9:[Click Me!](https://youtu.be/BXep_h8a7nY?si=BWVwMDX9v9g2CKTW)
