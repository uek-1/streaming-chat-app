 #!/bin/bash         
trap "exit" INT TERM ERR
trap "kill 0" EXIT

echo "Starting backend"
cd backend
cargo run &
BACKEND_ID=$!

echo "Starting frontend"
cd ../frontend
npm run dev &
FRONTEND_ID=$!

wait
