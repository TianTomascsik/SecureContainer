#!/bin/bash


echo "          _____                                ";
echo "         / ____|                               ";
echo "        | (___   ___  ___ _   _ _ __ ___       ";
echo "         \___ \ / _ \/ __| | | | '__/ _ \      ";
echo "         ____) |  __/ (__| |_| | | |  __/      ";
echo "        |_____/ \___|\___|\__,_|_|  \___|      ";
echo "   _____            _        _                 ";
echo "  / ____|          | |      (_)                ";
echo " | |     ___  _ __ | |_ __ _ _ _ __   ___ _ __ ";
echo " | |    / _ \| '_ \| __/ _\` | | '_ \ / _ \ '__|";
echo " | |___| (_) | | | | || (_| | | | | |  __/ |   ";
echo "  \_____\___/|_| |_|\__\__,_|_|_| |_|\___|_|   ";
echo "                                               ";
echo "                                               ";


############################################setting basic variables############################################
daemon=$(pwd)"/target/debug/secure_container_daemon"
cli=$(pwd)"/target/debug/secure_container_cli"

test_path="/home/tian/secure-container/Testing/"
size=200
mount_point=$test_path"MountME"
path_container=$test_path
namespace="ThisIsAContainerForTestingPurposes"
id="test"
path=$test_path"/"$namespace
secret="secret"
path2=$test_path"/path2"

set_up_test_environment() {
    mkdir -p $test_path
    mkdir -p $mount_point
    mkdir -p $path2
}

clean_up_test_environment() {
    sudo rm -rf $test_path
}

run_demo() {
     "$@" &
    pid_daemon=$!
}

echo "Setting up test environment"
set_up_test_environment


############################################test Create Container already exists############################################
run_demo sudo $daemon > /dev/null
sleep 5
echo "Test Create Container already exists"
$cli "create" "$size" "$mount_point" "$path_container" "$namespace" "$id" > /dev/null 2>&1
sleep 5
$cli "create" "$size" "$mount_point" "$path_container" "$namespace" "$id" > /dev/null 2>&1
exit_status=$?
if [ $exit_status -ne 23 ]; then
    echo -e "\e[31mFailed\e[0m: Test Create Container already exists Failed with code: $exit_status"
        else echo -e "\e[32mPassed\e[0m: Test Create Container already exists"
fi

kill -s SIGINT "$pid_daemon" > /dev/null 2>&1
sleep 5


############################################test Create Container already exists############################################
run_demo sudo $daemon > /dev/null
sleep 5
echo "Test Create Container already exists (Namespace)"
sleep 10
$cli "create" "$size" "$mount_point" "$path2" "$namespace" "$id" > /dev/null 2>&1
exit_status=$?
if [ $exit_status -ne 22 ]; then
    echo -e "\e[31mFailed\e[0m: Test Create Container already exists (Namespace) Failed with code: $exit_status"
        else echo -e "\e[32mPassed\e[0m: Test Create Container already exists (Namespace)"
fi
kill -s SIGINT "$pid_daemon" > /dev/null 2>&1
sleep 5


############################################test Open Container already Open############################################
run_demo sudo $daemon > /dev/null
sleep 5
echo "Test Open Container already Open"
sleep 10
$cli "open" "$mount_point" "$path" "$namespace" "$id" > /dev/null 2>&1
exit_status=$?
if [ $exit_status -ne 21 ]; then
    echo -e "\e[31mFailed\e[0m: Test Open Container already Open Failed with code: $exit_status"
        else echo -e "\e[32mPassed\e[0m: Test Open Container already Open"
fi
kill -s SIGINT "$pid_daemon" > /dev/null 2>&1
sleep 5


############################################test Export Container already Open############################################
run_demo sudo $daemon > /dev/null
sleep 5
echo "Test Export Container already Open"
sleep 10
$cli "export" "$path" "$namespace" "$id"  "$secret" > /dev/null 2>&1
exit_status=$?
if [ $exit_status -ne 21 ]; then
    echo -e "\e[31mFailed\e[0m: Test Export Container already Open Failed with code: $exit_status"
        else echo -e "\e[32mPassed\e[0m: Test Export Container already Open"
fi
$cli "close" "$mount_point" "$namespace" > /dev/null 2>&1
kill -s SIGINT "$pid_daemon" > /dev/null 2>&1
sleep 5

echo "Cleaning up test environment"
clean_up_test_environment
