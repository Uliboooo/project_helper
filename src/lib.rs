mod data_ctrl;

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    fs::{self},
    io::{self, BufWriter, Write},
};

enum PJError {
    SomeError,
    FailedGetHome,
    IoError(io::Error),
    FailedConvertT2Json,
    FailedConvertJson2T,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct TitleIndex {
    title_index: Vec<String>,
}
impl TitleIndex {
    fn new() -> Self {
        TitleIndex {
            title_index: Vec::<String>::new(),
        }
    }
    fn add(&mut self, title: String) {
        self.title_index.push(title);
    }
    fn new_key(&self) -> u64 {
        self.title_index.len() as u64
    }
    fn title_to_key(&self, title: String) -> Option<u64> {
        Some(self.title_index.iter().position(|t| t == &title)? as u64)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Relation {
    parent: Vec<u64>,
    sub_task: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum Status {
    NotStarted,
    InProgress,
    Done,
}
impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::NotStarted => write!(f, "not started"),
            Status::InProgress => write!(f, "in progress"),
            Status::Done => write!(f, "done"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    title: String,
    notes: Option<String>,
    due_date: Option<chrono::DateTime<Local>>,
    created_date: chrono::DateTime<Local>,
    relation: Option<Relation>,
    status: Status,
    project: Project,
    archived: bool,
}
impl Task {
    fn new() -> Self {
        Task {
            notes: None,
            due_date: None,
            created_date: chrono::Local::now(),
            relation: Some(Relation {
                parent: Vec::<u64>::new(),
                sub_task: Vec::<u64>::new(),
            }),
            status: Status::Done,
            project: Project {
                project: String::new(),
            },
            title: String::new(),
            archived: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    project: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Projects {
    projects: HashSet<String>,
}
impl Projects {
    fn new() -> Self {
        Projects {
            projects: HashSet::<String>::new(),
        }
    }
    fn add(&mut self, title: String) -> bool {
        // self.projects.push(title);
        self.projects.insert(title)
    }
    fn new_key(&self) -> u64 {
        self.projects.len() as u64
    }
    fn title_to_key(&self, title: String) -> Option<u64> {
        Some(self.projects.iter().position(|t| t == &title)? as u64)
    }
    fn rm(&mut self, target: String) -> bool {
        self.projects.remove(&target)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Tasks {
    tasks: HashMap<u64, Task>,
    index: TitleIndex,
    projects: Projects,
}
impl Tasks {
    fn new() -> Self {
        Tasks {
            tasks: HashMap::new(),
            index: TitleIndex::new(),
            projects: Projects::new(),
        }
    }
    fn add(&mut self, task: Task) {
        let key = self.index.new_key();
        self.tasks.insert(key, task);
    }

    fn rm(&mut self, target: String) -> Result<(), PJError> {
        self.tasks
            .get_mut(&self.index.title_to_key(target).ok_or(PJError::SomeError)?)
            .ok_or(PJError::SomeError)?
            .archived = true;
        Ok(())
    }

    fn delete(&mut self, target: String) -> Result<(), PJError> {
        self.tasks
            .remove(&self.index.title_to_key(target).ok_or(PJError::SomeError)?);
        Ok(())
    }

    fn get_info(&self) {
        todo!()
    }

    fn get_list(&self) {
        todo!()
    }

    fn add_project(&mut self, title: String) {
        self.projects.add(title);
    }
    fn rm_project(&mut self, target: String) -> Result<(), PJError> {
        if !self.projects.rm(target) {
            Err(PJError::SomeError)
        } else {
            Ok(())
        }
    }

    pub fn load() -> Result<Self, PJError> {
        let path_set = data_ctrl::initialize_helper_folder()?;

        let data_json = fs::read_to_string(path_set).map_err(PJError::IoError)?;

        Ok(if data_json.is_empty() {
            Tasks::new()
        } else {
            let parsed_tasks: Tasks =
                serde_json::from_str(&fs::read_to_string(&data_json).map_err(PJError::IoError)?)
                    .map_err(|_| PJError::FailedConvertJson2T)?;
            parsed_tasks
        })
    }

    pub fn save(&self) -> Result<(), PJError> {
        let path = data_ctrl::initialize_helper_folder()?;

        // 保存のためファイルは上書き。色々面倒なのでforでなく全部ベタ書き
        let mut tasks_writer = BufWriter::new(fs::File::create(path).map_err(PJError::IoError)?);

        // TODO: 空の場合の処理も?
        let tasks_json = serde_json::to_string(&self).map_err(|_| PJError::FailedConvertT2Json)?;

        write!(&mut tasks_writer, "{}", tasks_json).map_err(PJError::IoError)?;

        tasks_writer.flush().map_err(PJError::IoError)?;
        Ok(())
    }
}
