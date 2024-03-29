#!/bin/bash
# builds or runs rust project with passed path

set -e

if [ -z $1 ]; then
    echo "No path passed"
    exit 1
fi

if [ -z $2 ]; then
    echo "No action passed"
    exit 1
fi

case $2 in

    new)
        cd $1
        if [ -z $3 ]; then
            echo "No project name passed"
            exit 1
        fi
        cargo new $3
        ;;

    build)
        cd $1
        cargo build ${@:3}
        ;;
    
    check)
        cd $1
        cargo check ${@:3}
        ;;
    
    run)
        cd $1
        cargo run ${@:3}
        ;;

    doc)
        cd $1
        cargo doc ${@:3}
        ;;
    
    *)
        echo "Unknown action"
        exit 1
        ;;
esac
