A simple CLI based todo application built using Rust.It can perform basic operations like adding new task, marking tasks as completed as well as delete any task. The task entries are stores in a JSON file called as task.json. 

## Display the tasks.
![image](https://github.com/Parthiee/todo/assets/100670393/96caa1c7-5675-4e3e-92ff-c1fd05fb547b)
``` bash
cargo run -- show
```

## Add new tasks.
![image](https://github.com/Parthiee/todo/assets/100670393/b946bcb3-4525-43c1-8ec0-33bbb88b11ed)
``` bash
cargo run -- add TASK
```

## Mark any task as done.
![image](https://github.com/Parthiee/todo/assets/100670393/10521cab-8573-4f02-a4fa-2186675bf642)
``` bash
cargo run -- done INDEX_OF_TASK
```

## Delete any task
![image](https://github.com/Parthiee/todo/assets/100670393/16cff5f9-8ad8-4c6a-b9e2-3137550c6b9b)
``` bash
cargo run -- del INDEX_OF_TASK
```

