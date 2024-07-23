# ruwt
cli tool that creates web templates

# Usage

## universal config file
The first time ruwt is being used, it will create a universal config file at ~/.config/ruwt_config/config.toml where you can alter the default boilerplate code.

## After that
First you have to create a project with the command 'create-project', this command has some subcommands, like 'file-path' with which you can ad a file to the project folder while its being generated. There is also verbose, which gives you a verbos output of the directory structure of the project folder.

So to create a new project you run:
```ruwt create-project hello_world```
this will create a directory named hello_world.
with the following structure.

## Directory structure
```
Project/
|- index.html
|- serverconfig.toml
|- static/
   |- style.css
   |- index.js
```

In the serverconfig.toml file you can configure the projectfolder_path, which tells the server which files to serve. Keep in mind that the server needs a index.html file.
The ip_address and port is pretty self-explanatory.

## start server
With the command
```
ruwt -r
```
you start the server.
Keep in mind that the server needs to be started in the same place where the serverconfig.toml file is.
