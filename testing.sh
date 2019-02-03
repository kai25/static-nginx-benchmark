set -e

ADDRESS=35.210.147.86
REPO_NAME=static-nginx-benchmark
GATLING_BIN_PATH=~/soft/gatling/bin
SIMULATION_FOLDER=~/gatling_results/

remote_command () {
    ssh -i ~/.ssh/id_rsa vitaly_om25@$ADDRESS "$1"    
}

start_container () {
    echo "Starting container $1"
    folder_name="$1"
    remote_command "
        cd ~/testing/$REPO_NAME/$1;
        sudo docker stop $1 && sudo docker rm $1;
        sudo docker build -t "$1" .;
        sudo docker run -d --name $1 -p 80:80 $1;
    "
} 

run_gatling () {
    result_folder=$SIMULATION_FOLDER/$2/$1
    mkdir -p $result_folder
    JAVA_OPTS="-DfileName=$2.txt" $GATLING_BIN_PATH/gatling.sh -rf $result_folder -s computerdatabase.MySimulation
}


stop_container () {
    folder_name="$1"
    remote_command "
        sudo docker stop $1;
        sudo docker rm $1;
    "
} 

remote_command "
mkdir -p testing;
cd testing;
if [ ! -f $REPO_NAME ]; then
    rm -rf $REPO_NAME;
fi
git clone https://github.com/kai25/$REPO_NAME;
exit;
"

start_container "$1"

run_gatling $1 "med"

stop_container "$1"

remote_command "cd ~/testing; rm -rf $REPO_NAME"

echo 'OK'

