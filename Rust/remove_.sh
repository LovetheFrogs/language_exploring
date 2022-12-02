#!/bin/bash

if test $# -gt 1
then
	echo "Too many args."
	exit 1
fi

if test $# -eq 1
then
	FOLDER=$1
else
	echo "No folder specified."
	exit 2
fi

if [!-d "$FOLDER"];
then
	echo "Argument is not a folder."
	exit 3
else
	rm -rfv $FOLDER {*,.*}
	exit 0
fi
