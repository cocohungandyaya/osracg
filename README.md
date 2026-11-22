# osracg
A tools(CLI/GUI) to auto generate python/rust code file after recorded os keyboard and mouse actions

The file got generated is a standalone file which you can choose to rather import or run it

Full name:Os recorder action code generator

## Progress Log:
21 Nov 2026:Done the core part(press F12 to end recording),i also done the refactor part which format everyting into Duration:Action:param,also i make adjustment so then it read 0.1 seconds instead of 1 full seconds

22 Nov 2026:Done mapping out keys that will support and initialize the function generator,because i want ~play minecraft~ and today go hiking not much time to code so ~that is it today~ fahhh i spend 2 hours on fixing the refactor to support nanoseconds,no time for minecraft today :(
## Project Direction:
core->refactor->generator(involved env)->osracg(involved env)(CLI)->gui(tkinter)
*the refactor and generator both embed into the osracg(CLI)
