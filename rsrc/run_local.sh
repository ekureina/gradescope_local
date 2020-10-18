docker build -t local_gs {}/
docker create --name run_local_gs local_gs
docker start run_local_gs
docker exec -w /autograder run_local_gs /autograder/run_autograder
docker cp run_local_gs:/autograder/results/results.json \
    results.json
docker stop run_local_gs
docker rm run_local_gs
docker rm image local_gs
