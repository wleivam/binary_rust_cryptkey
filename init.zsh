#!/usr/bin/env zsh

declare -r binary="cryptkey"
declare -a combination # random order of constants
declare -r compiler="/usr/bin/rustc"
declare -a constants
declare -A env_constants=( # values must be executable commands
	[1]="date +%Y%m%d%H%M%S"
	[2]="hostname"
	[3]="whoami"
)
declare -r env_file="./src/env.rs"
declare -r main_file="./src/main.rs"
declare -r size=4000 # key file size eq 512KB approx with these constants

if [ ! -f $compiler ]
then
    print "Rust must be installed!"
    exit
fi

for x ({1..$size})
do
    combination+=$(echo ${(v)$(shuf --head-count=${#env_constants} --input-range=1-${#env_constants})} | sed "s/\ //g")
done
unset x

env_constants[0]="echo ${combination[*]} | sed 's/\ /,/g'"
unset combination

for constant in $(sort --numeric-sort <<< ${(kF)env_constants})
do
    constants+=\"$(sh -c $env_constants[$constant])\"
done
unset constant
unset env_constants

sed --in-place "s/\[.*\]/\[$(echo ${constants[*]} | sed 's/\ /,/g')\]/" $env_file
unset constants

$compiler $main_file -o $binary

sh -c ./$binary
