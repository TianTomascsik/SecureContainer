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
path=$test_path$namespace
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


############################################test Create and Open Container############################################
run_demo sudo $daemon
sleep 5
echo "Test Create and Open Container"
$cli "create" "$size" "$mount_point" "$path_container" "$namespace" "$id"
exit_status=$?
if [ $exit_status -ne 0 ]; then
    echo -e "\e[31mFailed\e[0m: Test Create and Open Container Failed with code: $exit_status"
    else echo -e "\e[32mPassed\e[0m: Test Create and Open Container"
fi
sleep 10
kill -s SIGINT "$pid_daemon" > /dev/null 2>&1
sleep 5


############################################test Close Container############################################
run_demo sudo $daemon
sleep 5
echo "Test Close Container"
$cli "close" "$mount_point" "$namespace"
exit_status=$?
if [ $exit_status -ne 0 ]; then
    echo -e "\e[31mFailed\e[0m: Test Close Container Failed with code: $exit_status"
        else echo -e "\e[32mPassed\e[0m: Test Close Container"
fi

kill -s SIGINT "$pid_daemon"
sleep 5






############################################test Export Container############################################
run_demo sudo $daemon
sleep 5
echo "Test Export Container"
$cli "export" "$path" "$namespace" "$id" "$secret"
exit_status=$?
if [ $exit_status -ne 0 ]; then
    echo -e "\e[31mFailed\e[0m: Test Export Failed with code: $exit_status"
        else echo -e "\e[32mPassed\e[0m: Test Export Container"
fi

kill -s SIGINT "$pid_daemon"
sleep 5

############################################test Import Container############################################
run_demo sudo $daemon
sleep 5
echo "Test Import Container"
$cli "import" "$path" "$namespace" "$id" "$secret"
exit_status=$?
if [ $exit_status -ne 0 ]; then
    echo -e "\e[31mFailed\e[0m: Test Import Container Failed with code: $exit_status"
        else echo -e "\e[32mPassed\e[0m: Test Import Container"
fi

kill -s SIGINT "$pid_daemon"
sleep 5

############################################test adding To auto Open############################################
run_demo sudo $daemon
sleep 5
echo "Test adding To auto Open"
$cli "add-auto-open" "$mount_point" "$path" "$namespace" "$id"
exit_status=$?
if [ $exit_status -ne 0 ]; then
    echo -e "\e[31mFailed\e[0m: Test adding To auto Open Failed with code: $exit_status"
        else echo -e "\e[32mPassed\e[0m: Test adding To auto Open"
fi

sleep 5


############################################test removing from auto Open############################################

echo "Test removing from auto Open"

$cli "remove-auto-open" "$mount_point" "$path" "$namespace" "$id"
exit_status=$?
if [ $exit_status -ne 0 ]; then
    echo -e "\e[31mFailed\e[0m: Test removing from auto Open Failed with code: $exit_status"
        else echo -e "\e[32mPassed\e[0m: Test removing from auto Open"
fi

kill -s SIGINT "$pid_daemon"
sleep 5


############################################test auto Open and Close############################################
echo "Test auto Open and Close Container"

sudo bash -c "echo "$mount_point","$path","$namespace","$id" >> "/usr/bin/auto_open""
sleep 5
run_demo sudo $daemon
exit_status=$?

sleep 40
kill -s SIGINT "$pid_daemon"
exit_status2=$?
if [ $exit_status -ne 0 ]; then
    echo -e "\e[31mFailed\e[0m: Test auto Open and Close Container Failed with code: $exit_status"
fi

sleep 60
if [ $exit_status2 -ne 0 ]; then
    echo -e "\e[31mFailed\e[0m: Test auto Open and Close Container Failed with code: $exit_status"
        else echo -e "\e[32mPassed\e[0m: Test auto Open and Close Container"
fi
sudo bash -c "head -n -1 /usr/bin/auto_open > temp.txt ; mv temp.txt /usr/bin/auto_open"






echo "Cleaning up test environment"
clean_up_test_environment