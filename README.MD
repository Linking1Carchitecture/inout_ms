## Ms
diesel setup
diesel migration run
diesel migration redo

cargo run

//------ docker file ----
(Corregir)
docker build -t ms_in_out .
docker run -it --rm --name ms_in_out ms_in_out