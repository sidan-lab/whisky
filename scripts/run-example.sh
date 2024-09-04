#!/bin/bash

# Function to handle termination of both processes
terminate_processes() {
    echo "Terminating processes..."
    kill $PID1
    kill $PID2
    wait $PID1
    wait $PID2
    echo "Processes terminated."
}

# Trap Ctrl-C (SIGINT) and call the terminate_processes function
trap terminate_processes SIGINT

# Start the first process
(
    cd ./example-web
    yarn
    yarn dev
) &
PID1=$!

# Start the second process
(
    cd ./packages/whisky-examples
    cargo run --bin whisky-examples
) &
PID2=$!

# Wait for both processes to finish
wait $PID1
wait $PID2