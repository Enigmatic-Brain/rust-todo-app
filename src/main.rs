use std::io;

fn main() {
    const OPTIONS_LIST: &str = r#" Welcome to TO-DO List CLI APPLICATION 
    Enter the command shown in brackets: 
    1) Add a Task (add)
    2) View Tasks (view)
    3) Update Task Status (update)
    4) Delete a Task (delete)
    5) Export Task list (export)
    6) Load Task List (load) "#;

    // enum Options {
    //     add,
    //     view,
    //     update,
    //     delete,
    //     export,
    //     load
    // }
    let mut tasks_list = Vec::new();

    loop {
        println!("{OPTIONS_LIST}");
        let mut command = String::new();
        let _ = io::stdin().read_line(&mut command);
        println!("command: {command}");
        
        if command.trim().to_lowercase().as_str()=="add" {
            add_a_task(&mut tasks_list)
        } else if command.trim().to_lowercase().as_str()=="view" {
            view_all_tasks(&tasks_list)
        } else if command.trim().to_lowercase().as_str()=="update" {
            update_a_task()
        } else if command.trim().to_lowercase().as_str()=="delete" {
            delete_a_task()
        } else if command.trim().to_lowercase().as_str()=="export" {
            export_a_task()
        } else {
            load_a_task()
        }
    }

}

enum Status {
    Active,
    Completed
}

struct TaskFormat {
    id: u32,
    description: String,
    status: Status
}


fn add_a_task(tasks_store: &mut Vec<TaskFormat>) {
    const REQUEST_FOR_DESC: &str = r#"Please enter the task description."#;
    println!("{REQUEST_FOR_DESC}");

    let mut task_description = String::new();
    let _ = io::stdin().read_line(&mut task_description);

    let id = {(tasks_store.len()+1).try_into().unwrap()};
    tasks_store.push(TaskFormat{
        id,
        description: task_description,
        status: Status::Active
    });
    println!("Task Added Successfully!")

}

fn view_all_tasks(tasks_store: &Vec<TaskFormat>) {
    
    println!("Here are all the tasks: ");
    for task in tasks_store {
        let task_id = task.id;
        let description = &task.description;
        let status = match task.status{
            Status::Active => "Active",
            Status::Completed => "Completed"
        };
        println!("{task_id}-->{description} | {status}");
    }

}

fn update_a_task() {
    println!("Updating the task")
}

fn delete_a_task() {
    println!("deleting a task")
}

fn export_a_task() {
    println!("exporting a task")
}

fn load_a_task() {
    println!("loading a task")
}