## Ms
diesel setup
diesel migration run
diesel migration redo

cargo run

//------ docker file ----

docker pull rust
docker build -t my-rust-app .
docker run -it --rm --name my-running-app my-rust-app