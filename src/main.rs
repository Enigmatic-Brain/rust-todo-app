use std::io::{self};

fn main() {
    const OPTIONS_LIST: &str = r#" Welcome to TO-DO List CLI APPLICATION 
    Enter the command shown in brackets: 
    1) Add a Task (add)
    2) View Tasks (view)
    3) Update Task Status (update)
    4) Delete a Task (delete) "#;

    // enum Options {
    //     add,
    //     view,
    //     update,
    //     delete,
    //     export,
    //     load
    // }
    let mut tasks_list = Vec::new();
    let mut id_counter: u32 = 0;
    loop {
        println!("{OPTIONS_LIST}");
        let mut command = String::new();
        let _ = io::stdin().read_line(&mut command);
        println!("\n");
        // println!("command: {command}");
        let input = command.trim().to_ascii_lowercase();
        match input.as_str() {
            "add"=>{
                add_a_task(&mut tasks_list, id_counter);
                id_counter = id_counter + 1;
            },
            "view" => view_all_tasks(&tasks_list),
            "update" => update_a_task(&mut tasks_list),
            "delete" => delete_a_task(&mut tasks_list),
            "exit" => {
                println!("Exiting....");
                break;
            },
            _ => {
                println!("Didn't recognise: {}, please enter commands from the option shown:\n", command.trim())
            }
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


fn add_a_task(tasks_store: &mut Vec<TaskFormat>, counter: u32) {
    const REQUEST_FOR_DESC: &str = r#"Please enter the task description."#;
    println!("{REQUEST_FOR_DESC}");

    let mut task_description = String::new();
    let _ = io::stdin().read_line(&mut task_description);

    let id = counter + 1;
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


fn update_a_task(tasks_store: &mut Vec<TaskFormat>) {
    println!("Please enter the task ID to mark it as complete");
    let mut task_id = String::new();
    let _ = io::stdin().read_line(&mut task_id);

    let parsed_id = match task_id.trim().parse::<u32>(){
        Ok(id) => id,
        Err(_) => {
            println!("Please enter a valid task ID");
            return;
        }
    };

    if let Some(position) = tasks_store.iter().position(|task| task.id==parsed_id){
        let matched_description = &tasks_store[position];
        let description = &matched_description.description;
            tasks_store[position].status = match matched_description.status {
                Status::Completed => {
                    println!("It is already marked Completed");
                    Status::Completed
                },
                Status::Active => {
                    println!("{description} marked Completed");
                    Status::Completed
                }
            }
    } else {
        println!("No task found with the given ID.")
    }
    
}


fn delete_a_task(tasks_store: &mut Vec<TaskFormat>) {

    println!("Please provide the task id to delete:");
    let mut to_delete_task_id: String = String::new();
    let _ = io::stdin().read_line(&mut to_delete_task_id);
    
    let parsed_id = match to_delete_task_id.trim().parse::<u32>() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid Task Id. Please check the task id");
            return;
        }
    };
    
    if let Some(index) = tasks_store.iter().position(|task| task.id==parsed_id){
        let removed_task = tasks_store.remove(index);
        println!("Deleted task {}: {}", removed_task.id, removed_task.description);
    } else {
        println!("No task found with ID: {}", to_delete_task_id);
    }
}