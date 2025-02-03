mod data_ctrl;

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    fs::{self},
    io::{self, BufWriter, Write},
};

pub enum PJError {
    SomeError,
    FailedGetHome,
    IoError(io::Error),
    FailedConvertT2Json,
    FailedConvertJson2T,
    NotFoundItem,
    NotFoundKey,
    FailedRemoveItem,
    TasksIsEmpty,
}
impl Display for PJError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PJError::SomeError => write!(f, "something error"),
            PJError::FailedGetHome => write!(f, "failed get home path.please restart."),
            PJError::IoError(e) => write!(f, "IO error: {}", e),
            PJError::FailedConvertT2Json => write!(f, "failed convert tasks to json with serde."),
            PJError::FailedConvertJson2T => write!(f, "failed convert json to tasks with serde."),
            PJError::NotFoundItem => write!(f, "not found item. please input exist task."),
            PJError::NotFoundKey => write!(f, "not found key. please input exist task."),
            PJError::FailedRemoveItem => write!(f, "failed remove item for some error"),
            PJError::TasksIsEmpty => write!(
                f,
                "Tasks and projects is nothing. so don't sore data to database. "
            ),
        }
    }
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
    fn add(&mut self, title: String) -> u64 {
        self.title_index.push(title);
        self.title_index.len() as u64
    }
    // fn new_key(&self) -> u64 {
    //     self.title_index.len() as u64
    // }
    fn title_to_key(&self, title: String) -> Option<u64> {
        Some(self.title_index.iter().position(|t| t == &title)? as u64)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Relation {
    parent: Vec<u64>,
    sub_task: Vec<u64>,
}
impl Relation {
    pub fn new() -> Self {
        Relation {
            parent: Vec::<u64>::new(),
            sub_task: Vec::<u64>::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Status {
    NotStarted,
    InProgress,
    Done,
}
impl Status {
    pub fn new() -> Self {
        Self::NotStarted
    }
}
impl Default for Status {
    fn default() -> Self {
        Self::NotStarted
    }
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
pub struct Task {
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
    pub fn new(
        title: String,
        notes: Option<String>,
        due_date: Option<chrono::DateTime<Local>>,
        relation: Option<Relation>,
        status: Status,
        pj: Project,
        archive: bool,
    ) -> Self {
        Task {
            title,
            notes,
            due_date,
            created_date: chrono::Local::now(),
            relation,
            status,
            project: pj,
            archived: archive,
        }
    }
    // fn new_emp() -> Self {
    //     Task {
    //         notes: None,
    //         due_date: None,
    //         created_date: chrono::Local::now(),
    //         relation: Some(Relation {
    //             parent: Vec::<u64>::new(),
    //             sub_task: Vec::<u64>::new(),
    //         }),
    //         status: Status::Done,
    //         project: Project {
    //             project: String::new(),
    //         },
    //         title: String::new(),
    //         archived: true,
    //     }
    // }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Project {
    project: String,
}

impl Project {
    pub fn new() -> Self {
        Project {
            project: String::new(),
        }
    }
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

    // fn new_key(&self) -> u64 {
    //     self.projects.len() as u64
    // }

    // fn title_to_key(&self, title: String) -> Option<u64> {
    //     Some(self.projects.iter().position(|t| t == &title)? as u64)
    // }

    fn rm(&mut self, target: String) -> bool {
        self.projects.remove(&target)
    }
}

#[cfg(feature = "v2")]
pub enum EditTarget {
    Title,
    Notes,
    Due,
    Relation,
    Status,
    Project,
    Archive,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    tasks: HashMap<u64, Task>,
    index: TitleIndex,
    projects: Projects,
}
impl Tasks {
    // Creates a new `Tasks` struct.
    // fn new() -> Self {
    //     Tasks {
    //         tasks: HashMap::new(),
    //         index: TitleIndex::new(),
    //         projects: Projects::new(),
    //     }
    // }

    fn new_emp() -> Self {
        Tasks {
            tasks: HashMap::new(),
            index: TitleIndex::new(),
            projects: Projects::new(),
        }
    }

    /// Adds a task to the list of tasks
    pub fn add_task(&mut self, task: Task) {
        // let key = self.index.new_key();
        let key = self.index.add(task.title.clone());
        self.tasks.insert(key, task);
    }

    /// let tasks's archive be true. don't delete & possible to put back.
    pub fn archive_task(&mut self, target: String) -> Result<(), PJError> {
        self.tasks
            .get_mut(
                &self
                    .index
                    .title_to_key(target)
                    .ok_or(PJError::NotFoundKey)?,
            )
            .ok_or(PJError::NotFoundItem)?
            .archived = true;
        Ok(())
    }

    /// removed(rm) tasks put back
    pub fn back_task(&mut self, target: String) -> Result<(), PJError> {
        self.tasks
            .get_mut(
                &self
                    .index
                    .title_to_key(target)
                    .ok_or(PJError::NotFoundKey)?,
            )
            .ok_or(PJError::NotFoundItem)?
            .archived = false;
        Ok(())
    }

    /// delete task. can't put back the deleted task.
    pub fn delete_task(&mut self, target: String) -> Result<(), PJError> {
        self.tasks.remove(
            &self
                .index
                .title_to_key(target)
                .ok_or(PJError::NotFoundKey)?,
        );
        Ok(())
    }

    // #[doc(cfg(feature = "v2"))]
    #[cfg(feature = "v2")]
    pub fn edit(&mut self, target: EditTarget) {
        match target {
            EditTarget::Title => todo!(),
            EditTarget::Notes => todo!(),
            EditTarget::Due => todo!(),
            EditTarget::Relation => todo!(),
            EditTarget::Status => todo!(),
            EditTarget::Project => todo!(),
            EditTarget::Archive => todo!(),
        }
    }

    #[cfg(feature = "v2")]
    fn get_info(&self) {
        todo!()
    }

    #[cfg(feature = "v2")]
    fn get_list(&self) {
        todo!()
    }

    pub fn add_project(&mut self, title: String) {
        self.projects.add(title);
    }

    pub fn rm_project(&mut self, target: String) -> Result<(), PJError> {
        if !self.projects.rm(target) {
            Err(PJError::FailedRemoveItem)
        } else {
            Ok(())
        }
    }

    /// Load tasks from JSON file into memory.
    ///
    /// Returns:
    /// - `Ok(Tasks)`: A new instance of Tasks containing loaded or parsed task data
    ///   if a JSON file exists in the initialized helper folder.
    /// - `Err(PJError)`: An error if no JSON file is found or if deserialization fails.
    ///
    /// If the JSON file is empty, a new Task instance will be created and returned.
    /// If deserialization of the JSON fails (e.g., invalid format), an error will
    /// be returned with details about the parsing failure.
    ///
    pub fn load() -> Result<Self, PJError> {
        let path_set = data_ctrl::initialize_helper_folder()?;

        let data_json = fs::read_to_string(path_set).map_err(PJError::IoError)?;

        Ok(if data_json.is_empty() {
            Tasks::new_emp()
        } else {
            let parsed_tasks: Tasks =
                serde_json::from_str(&fs::read_to_string(&data_json).map_err(PJError::IoError)?)
                    .map_err(|_| PJError::FailedConvertJson2T)?;
            parsed_tasks
        })
    }

    /// # save()
    ///
    /// Saves task data into a JSON file in a specific folder path.
    ///
    /// # Return Value
    /// Returns Ok(()) on success, or PJError on failure with appropriate error message
    ///
    /// # Possible Errors
    /// - Path not found in data_ctrl::initialize_helper_folder()
    /// - Failure to convert task data to JSON string
    /// - IO errors while writing files
    ///
    /// # Implementation Details
    /// - Uses BufWriter for efficient buffer-based writing of large files
    /// - Each write operation overwrites the file entirely instead of appending
    pub fn save(&self) -> Result<(), PJError> {
        let path = data_ctrl::initialize_helper_folder()?;

        // 保存のためファイルは上書き。色々面倒なのでforでなく全部ベタ書き
        let mut tasks_writer = BufWriter::new(fs::File::create(path).map_err(PJError::IoError)?);

        // TODO: 空の場合の処理も?
        let tasks_json = serde_json::to_string(&self).map_err(|_| PJError::FailedConvertT2Json)?;
        if tasks_json.is_empty() {
            return Err(PJError::TasksIsEmpty);
        }

        write!(&mut tasks_writer, "{}", tasks_json).map_err(PJError::IoError)?;

        tasks_writer.flush().map_err(PJError::IoError)?;
        Ok(())
    }
}

impl Default for Tasks {
    fn default() -> Self {
        Tasks::new_emp()
    }
}
