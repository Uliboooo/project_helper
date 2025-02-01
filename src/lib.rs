mod data_ctrl;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    fs::{self},
    io::{self, BufWriter, Write},
};

#[derive(Debug)]
pub enum PJHelpError {
    SomeError,
    FailedGetHome,
    IoError(io::Error),
    FailedConvertT2Json,
    FailedConvertJson2T,
}

impl Display for PJHelpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PJHelpError::SomeError => write!(f, ""),
            PJHelpError::FailedGetHome => write!(f, ""),
            PJHelpError::IoError(error) => write!(f, "{}", error),
            PJHelpError::FailedConvertT2Json => write!(f, "failed convert type of Data to json."),
            PJHelpError::FailedConvertJson2T => write!(f, "failed convert json to type of Data."),
        }
    }
}

/// A task struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    title: String,
    id: u64,
    notes: Option<String>,
    status: Status,
    parent_id: Option<u64>,
    subtasks: Vec<u64>,
    created_date: DateTime<Local>,
    due_date: Option<DateTime<Local>>,
    project: Project,
}

/// Task implementation
impl Task {
    pub fn new(
        title: String,
        notes: Option<String>,
        status: Status,
        parent_id: Option<u64>,
        subtasks: Vec<u64>,
        due_date: Option<DateTime<Local>>,
        project: Project,
    ) -> Self {
        Task {
            title,
            id: 3,
            notes,
            status,
            parent_id,
            subtasks,
            created_date: Local::now(),
            due_date,
            project,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    NotStarted,
    InProgress,
    Done,
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "n" => Status::NotStarted,
            "i" => Status::InProgress,
            "d" => Status::Done,
            _ => Status::NotStarted,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tasks {
    tasks: Vec<Task>,
}
impl Tasks {
    fn new() -> Self {
        Tasks { tasks: Vec::new() }
    }
    pub fn load_or_init() -> Self {
        // data_ctrl::initialize_helper_folder();
        Tasks { tasks: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    name: String,
    id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Projects {
    projects: Vec<Project>,
}
impl Projects {
    fn new() -> Self {
        Projects {
            projects: Vec::new(),
        }
    }
    fn new_id(&self) -> u64 {
        self.projects.len() as u64
    }
    fn add(&mut self, project: Project) {
        self.projects.push(project);
    }
}

#[derive(Debug)]
pub struct Data {
    tasks: Tasks,
    projects: Projects,
}

impl Data {
    pub fn load() -> Result<Self, PJHelpError> {
        let path_set = data_ctrl::initialize_helper_folder()?;

        let tasks_json = fs::read_to_string(path_set.0).map_err(PJHelpError::IoError)?;
        let projects_json = fs::read_to_string(path_set.1).map_err(PJHelpError::IoError)?;

        Ok(Data {
            tasks: if tasks_json.is_empty() {
                Tasks::new()
            } else {
                let v: Tasks =
                    serde_json::from_str(&tasks_json).map_err(|_| PJHelpError::FailedGetHome)?;
                v
            },
            projects: if projects_json.is_empty() {
                Projects::new()
            } else {
                let v: Projects =
                    serde_json::from_str(&projects_json).map_err(|_| PJHelpError::FailedGetHome)?;
                v
            },
        })
    }
    pub fn save(&self) -> Result<(), PJHelpError> {
        let (tasks_path, projects_path) = data_ctrl::initialize_helper_folder()?;

        let mut tasks_writer =
            BufWriter::new(fs::File::create(tasks_path).map_err(PJHelpError::IoError)?);
        let mut pjs_writer =
            BufWriter::new(fs::File::create(projects_path).map_err(PJHelpError::IoError)?);

        // TODO: 空の場合の処理も?
        let tasks_json =
            serde_json::to_string(&self.tasks).map_err(|_| PJHelpError::FailedConvertT2Json)?;
        let projects_json =
            serde_json::to_string(&self.projects).map_err(|_| PJHelpError::FailedConvertT2Json)?;

        write!(&mut tasks_writer, "{}", tasks_json).map_err(PJHelpError::IoError)?;
        write!(&mut pjs_writer, "{}", projects_json).map_err(PJHelpError::IoError)?;

        tasks_writer.flush().map_err(PJHelpError::IoError)?;
        pjs_writer.flush().map_err(PJHelpError::IoError)?;

        Ok(())
    }
}
