use std::collections::HashMap;

use crate::{application::gateways::TodoRepository, domain::Todo};

#[derive(Debug, Clone)]
pub struct TodoRepoInMem {
    data: HashMap<String, Todo>,
}

impl TodoRepository for TodoRepoInMem {
    fn create_todo(&mut self, todo: Todo) {
        self.add_todo(todo.id.clone().unwrap(), todo)
    }

    fn delete_todo(&mut self, id: String) {
        match self.data.remove(&id) {
            Some(_) => self.flush(),
            None => (),
        }
    }

    fn update_todo(&mut self, todo: Todo) {
        let id = todo.id.clone().unwrap();
        let old_todo = match self.data.get(&id) {
            Some(vl) => vl,
            None => {
                println!("todo not find");
                return;
            }
        };
        let new_todo = Todo {
            id: old_todo.clone().id,
            title: match todo.title {
                Some(vl) => Some(vl),
                None => old_todo.clone().title,
            },
            description: match todo.description {
                Some(vl) => Some(vl),
                None => old_todo.clone().description,
            },
            color: match todo.color {
                Some(vl) => Some(vl),
                None => old_todo.clone().color,
            },
            is_completed: match todo.is_completed {
                Some(vl) => Some(vl),
                None => old_todo.is_completed,
            },
        };
        self.data.remove(&id);
        self.add_todo(id, new_todo);
        self.flush();
    }

    fn read_todo(&mut self, id: String) -> Option<Todo> {
        let result = self.data.get_mut(&id).cloned();
        result
    }

    fn read_todos(&mut self, req: crate::application::dto::ReadTodosReq) -> Vec<Todo> {
        let mut data: Vec<Todo> = vec![];
        for i in self.data.clone().into_iter() {
            if req.all.is_some() {
                data.push(i.1);
            } else if req.completed.is_some() && req.color.is_some() {
                if req.completed.unwrap() == i.clone().1.is_completed.unwrap()
                    && i.clone().1.color.unwrap() == req.color.clone().unwrap()
                {
                    data.push(i.1);
                }
            } else if req.color.is_some() {
                if i.clone().1.color.unwrap() == req.color.clone().unwrap() {
                    data.push(i.1);
                }
            } else if req.completed.is_some() {
                if req.completed.unwrap() == i.clone().1.is_completed.unwrap() {
                    data.push(i.1);
                }
            }
        }
        data
    }
}

impl TodoRepoInMem {
    pub fn new() -> Self {
        if std::fs::read_to_string("todos.json").is_err() {
            std::fs::File::create("todos.json").unwrap();
        }

        let new_data = std::fs::read_to_string("todos.json").unwrap();
        if new_data.is_empty() {
            return Self {
                data: HashMap::<String, Todo>::new(),
            };
        }
        let data: HashMap<String, Todo> = serde_json::from_str(&new_data).unwrap();
        Self { data }
    }

    pub fn add_todo(&mut self, id: String, todo: Todo) {
        self.data.insert(id, todo);
        self.flush();
    }

    fn flush(&mut self) {
        let d = self.data.clone();
        match serde_json::to_string(&d) {
            Ok(vl) => {
                std::fs::write("todos.json", vl).unwrap();
            }
            Err(_) => {
                panic!("can not wirte todo in file");
            }
        };
    }
}
