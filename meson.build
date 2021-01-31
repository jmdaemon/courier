project('courier', 'c')

gtk4 = dependency('gtk4')

includes = include_directories('include')
executable('courier', 'src/ui/main.c', 'src/ui/app.c', 'src/ui/appwin.c', dependencies : gtk4, include_directories : includes)
