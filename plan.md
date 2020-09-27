# Kubeconfig CLI tool

## Purpose
- To manage the use of multiple kube config files

## Requirements
- load config file from defaulted directory
	- Need to figure out where we are going to place this. 
	- Would like to honor XDG_CONFIG_DIRS
- list my kubeconfig files 
- delete my kubeconfig files
- switch my kubeconfig files
- import my kubeconfig in to the config directory
- list helpful tips / commands
- show my active kubeconfig
- configure tool
	- Get user input for the following:
		- config file directory [String]
			- Need to convert from string to path
			- Need to validate path
			- If file doesn't exist we should create the file
			- Overwrite the file each time we do a config
	- Set kubeconfig file from $KUBECONFIG env var. 

## Commands
- list
	- list all configs in the tool config file
- delete
	- INPUT
		- remove the specified kubeconfig from the kcctl's config and the kcctl config dir.
	- --current
		- remove the current config from the config dir
- switch
	- remove the current config
	- copy the config from the config directory to ./kube/config
- import 
  - copy the config file from ./kube/config to the config directory
- show
	- INPUT
		- STRING
		- prints the config file named $INPUT to std_out
	- --current
- edit
	- INPUT
	  - Opens the config file named $INPUT with $EDITOR
- purge
	- Removes all configs
- context
	- list
	- switch
		-	INPUT


## Constants 
config_path = check if $KUBECONFIG is set use this as the target if not default to ./kube/
config_dir =